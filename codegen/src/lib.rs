pub mod util;

use std::{collections::{HashMap, HashSet, VecDeque}, path::{Path, PathBuf}, str::FromStr};
use std::fs;

use proc_macro2::TokenStream;
use ree_lib::il2cpp::*;
use quote::{quote, format_ident};
use util::TypeName;

pub trait Gen<T = ()>: Sized {
    fn generate(&self, ctx: &T) -> (TokenStream, HashSet<String>);
}

impl<'a> Gen<Il2Cpp<'a>> for REType<'a> {
    fn generate(&self, ctx: &Il2Cpp<'a>) -> (TokenStream, HashSet<String>) {
        let type_name = TypeName::parse(self.name);
        let (fields, deps) = generate_struct_fields(self, ctx);
        // TODO: treat enums differently

        // name of the struct, it should be after the last period, and before generics
        // TODO: include flattened generics in this name
        let struct_name = type_name.struct_name();

        // a panic is too severe here, should just generate a warning and continue to the next type
        // maybe just return no tokens, no deps, or i could actually include dependencies why not
        let struct_name = TokenStream::from_str(&struct_name)
            .unwrap_or_else(|_| panic!("Failed to parse struct name {}", struct_name));

        let size = self.size.saturating_sub(0x10);
        let size_lit = proc_macro2::Literal::usize_unsuffixed(size as usize);
        let tokens = quote! {
            #[repr(C)]
            pub struct #struct_name {
                #fields
            }
            const _: () = assert!(std::mem::size_of::<#struct_name>() == #size_lit);
        };
        (tokens, deps)
    }
}

pub fn generate_struct_fields<'a>(ty: &REType<'a>, il2cpp: &Il2Cpp<'a>) -> (TokenStream, HashSet<String>) {
    let mut deps = HashSet::new();
    let mut fields = Vec::new();
    let mut pad_count = 0;
    // TODO: recursively add parent's field to this, or add a __super/__base field with an inlined parent
    let mut valid_fields: Vec<_> = ty.fields.values()
        .filter(|f| !f.flags.contains(&REFieldFlag::Static))
        .collect();

    // sorts as a tuple so that field at the same offset are ordered largest to smallest
    // TODO: possibly add unions, or just take the first largest field at the same address?
    // also might have to sometimes deal with overlapping fields that aren't at the same offset
    valid_fields.sort_by_cached_key(|f| {
        let size = il2cpp
            .get(f.r#type)
            .map_or(0, |ty| util::get_field_size(f, ty));
        (f.offset_from_base, std::cmp::Reverse(size))
    });

    // start at field ptr, this is correct, but it feels wrong since im using field.offset_from_base
    let mut current_offset: usize = 0x10;

    for field in valid_fields {
        log::debug!("{:?}", field);
        deps.insert(field.r#type.to_string());

        let Some(field_ty) = il2cpp.get(field.r#type) else { continue };

        let field_ty_name = TypeName::parse(field.r#type);

        // TODO: add generics to the fields
        if !field_ty_name.generics.is_empty() {
            log::warn!("Skipping field {}: {} in {}", field.name, field.r#type, ty.name);
            continue;
        }

        let is_array = field_ty.parent == "System.Array";
        let is_valuetype = field_ty.parent == "System.ValueType";
        let is_enum = field_ty.parent == "System.Enum";
        let is_reference_type = !(is_valuetype || is_enum);

        // the rust type name, depends on where the structs module will be placed
        // this would mean something like;
        // snow.player.PlayerManager -> crate::snow::player::PlayerManager
        // and then the struct PlayerManager tokens would be in snow/player/mod.rs or snow/player.rs
        let rust_type_path = field_ty_name.qualified_struct_path();

        let mut field_ty_str = rust_type_path;
        // wrap it as an array which has a known layout (it becomes a [T; 0] at the end of the 0x20
        // sized Array struct), this also ends up getting wrapped in a ManagedPtr since it's not a
        // reference type (TODO: make sure this is ALWAYS correct)
        if is_array {
            field_ty_str = format!("reframework::regen::Array<{field_ty_str}>");
        }

        let field_size = util::get_field_size(field, field_ty) as usize;

        // TODO: wtf? doesn't align with hwat i ahve for current_offset starting at  
        let offset = field.offset_from_base as usize;

        if offset > current_offset {
            let spacing = offset - current_offset;
            let spacing_lit = proc_macro2::Literal::usize_unsuffixed(spacing);
            let pad_name = format_ident!("_pad{pad_count}");
            fields.push(quote! { #pad_name: [u8; #spacing_lit] });
            pad_count += 1;
        } else if offset < current_offset {
            log::warn!("Overlap detected at field {} (offset {})", field.name, offset);
            continue;
        }

        let clean_name = util::clean_name(field.name);
        // all fields are prefixed with r# so i don't have to worry about stupid garbage
        // TODO: maybe same thing for struct paths?
        let field_name = syn::Ident::new_raw(&clean_name, proc_macro2::Span::call_site());
        let field_ty_tokens = TokenStream::from_str(&field_ty_str).expect("Failed to parse field type");
        let field_ty_tokens = if is_reference_type {
            quote! { reframework::regen::ManagedPtr<#field_ty_tokens> }
        } else {
            quote! { #field_ty_tokens }
        };
        fields.push(quote! { pub #field_name: #field_ty_tokens });
        current_offset = offset + field_size;
    }

    // final padding to the struct size
    let offset = ty.size as usize;
    if offset > current_offset {
        let spacing = offset - current_offset;
        let spacing_lit = proc_macro2::Literal::usize_unsuffixed(spacing);
        let pad_name = format_ident!("_pad_end");
        fields.push(quote! { #pad_name: [u8; #spacing_lit] });
    } else if offset < current_offset {
        // this probably should never have a chance of happening
        log::warn!("Overlap detected at end of struct (offset {})", offset);
    }

    let fields = quote! { #(#fields),* };
    (fields, deps)
}

// holder for a structs generated tokens, full name, and dependencies
pub struct Generated {
    pub name: String,
    pub namespace: Vec<String>,
    pub tokens: TokenStream,
    pub deps: HashSet<String>,
}

pub struct Generator<'a> {
    roots: Vec<String>,
    il2cpp: &'a Il2Cpp<'a>,
    target_path: PathBuf,
}

impl<'a> Generator<'a> {
    pub fn new(roots: Vec<String>, target_path: &str, il2cpp: &'a Il2Cpp<'a>) -> Self {
        Self {
            roots,
            il2cpp,
            target_path: target_path.into(),
        }
    }

    pub fn generate(&self) {
        if let Ok(mut read_dir) = self.target_path.read_dir() {
            while let Some(e) = read_dir.next() {
                if let Ok(e) = e {
                    if e.file_name() == "lib.rs" {
                        log::warn!("lib.rs already exists in the target path, make sure to clear out old files")
                    }
                }
            }
        }

        let mut generated: HashMap<&str, Generated> = HashMap::new();
        let mut queue = VecDeque::new();
        for root in &self.roots {
            queue.push_back(root.to_string());
        }

        log::info!("Generating tokens");
        while let Some(ty) = queue.pop_front() {
            if generated.contains_key(ty.as_str()) {
                continue;
            }
            if let Some(ty) = self.il2cpp.get(ty.as_str()) {
                let name = TypeName::parse(ty.name);

                // TODO: Skip generic types for now
                // should be changed to flatten struct names
                // i.e. Foo.Bar<Foo.Baz> -> foo::Bar_Foo_Baz
                // OR figure out a way to keep generics (harder)
                if !name.generics.is_empty() {
                    continue;
                }

                let (tokens, deps) = ty.generate(self.il2cpp);

                for dep in &deps {
                    if !generated.contains_key(dep.as_str()) && !queue.contains(dep) {
                        queue.push_back(dep.to_string());
                    }
                }

                let generated_type = Generated {
                    name: ty.name.to_string(),
                    namespace: name.hierarchy.iter().map(|v| v.to_string()).collect(),
                    deps,
                    tokens,
                };
                generated.insert(ty.name, generated_type);
            }
        }

        log::info!("Generating file tree");
        let mut root = ModuleNode::default();
        for (_name, ty) in generated {
            let mut current_node = &mut root;
            let namespace_dirs = ty.namespace.iter().take(ty.namespace.len().saturating_sub(1));
            for part in namespace_dirs {
                let mod_name = part.to_lowercase();
                current_node = current_node.submodules.entry(mod_name).or_default();
            }
            current_node.structs.push(ty);
        }

        log::info!("Writing to path {}", self.target_path.to_string_lossy());
        root.write_to_disk(&self.target_path, "lib");

        util::format_generated_code(&self.target_path);
    }
}

#[derive(Default)]
pub struct ModuleNode {
    pub submodules: HashMap<String, ModuleNode>,
    pub structs: Vec<Generated>,
}

impl ModuleNode {
    pub fn write_to_disk(&self, current_dir: &Path, node_name: &str) {
        let is_root = node_name == "lib";
        let target_dir = if is_root {
            current_dir.to_path_buf()
        } else {
            current_dir.join(node_name)
        };

        fs::create_dir_all(&target_dir).unwrap_or_else(|e| {
            panic!("Failed to create directory {}: {}", target_dir.display(), e);
        });

        let mut file_content = quote! {
            #![allow(non_camel_case_types, non_snake_case, unused_imports, dead_code)]
        };

        for submod_name in self.submodules.keys() {
            let ident = format_ident!("{}", submod_name);
            file_content.extend(quote! { pub mod #ident; });
        }

         for s in &self.structs {

            let tokens = &s.tokens;
            /*let struct_file_content = quote! {
                #tokens
            };*/
            file_content.extend( quote! { #tokens });
            //let struct_dir = target_dir.join(mod_name);
            //fs::create_dir_all(&struct_dir).unwrap_or_else(|e| {
            //    panic!("Failed to create struct dir {}: {}", struct_dir.display(), e);
            //});

            //fs::write(struct_dir.join("mod.rs"), struct_file_content.to_string()).unwrap();
        }

        let file_name = if is_root { "lib.rs" } else { "mod.rs" };
        fs::write(target_dir.join(file_name), file_content.to_string()).unwrap();

        for (name, node) in &self.submodules {
            node.write_to_disk(&target_dir, name);
        }
    }
}


