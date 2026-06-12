use thiserror::Error;

use crate::rsz::{Rsz, RszMap, Value};

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Could not parse index: {0}")]
    ParseIndexError(#[from] std::num::ParseIntError),
    #[error("Could not find Index {0}")]
    InvalidIndex(usize),
    #[error("Could not find label {0}")]
    InvalidLabel(String),
}

pub enum QueryNode {
    Label(String),
    Index(usize),
}

impl QueryNode {
    pub fn index(&self) -> Option<usize> {
        if let QueryNode::Index(index) = self {
            return Some(*index)
        }
        None
    }

    pub fn label(&self) -> Option<&str> {
        if let QueryNode::Label(label) = self {
            return Some(label)
        }
        None
    }
}

pub struct Query {
    pub query: Vec<QueryNode>
}

impl TryFrom<&str> for Query {
    type Error = QueryError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // split on periods
        let mut res = Vec::new();
        for label in value.split('.') {
            match (label.find('['), label.rfind(']')) {
                (Some(start), Some(end)) if start < end => {
                    res.push(QueryNode::Label(label[..start].to_string()));
                    let number = &label[start + 1..end];
                    let index = number.parse::<usize>()?;
                    res.push(QueryNode::Index(index));
                }
                _ => {
                    res.push(QueryNode::Label(label.to_string()));
                }
            };
        }
        Ok(Self {
            query: res,
        })
    }
}

impl Rsz {

    // this requires that path length is equal to or greater than 1 (not including the an intiial
    // root indexing)
    pub fn get<'a>(&'a self, path: &str, rsz_map: &RszMap) -> Option<&'a Value> {
        let query = Query::try_from(path).ok()?;
        // TODO: add a debug assert or some shit like that
        if query.query.is_empty() { // this is technically borked because of how im parseing queries
            return None; 
        }

        let mut iter = query.query.iter().peekable();
        let root_index = iter
            .next_if(|node| node.index().is_some())
            .and_then(|node| node.index())
            .unwrap_or(0);
        let root_index = *self.roots.get(root_index)?;

        let mut cur_value: Option<&'a Value> = None;
        for node in iter {
            match node {
                QueryNode::Label(label) => {
                    let obj_id = match cur_value {
                        Some(Value::Object(id)) => *id,
                        None => root_index,
                        _ => return None,
                    };

                    let current_instance = self.instances.get(obj_id as usize)?;
                    let type_info = rsz_map.get_by_hash(current_instance.hash)?; 
                    let field_index = type_info.get_field_idx(label)?;
                    let value = current_instance.fields.get(field_index)?;
                    cur_value = Some(value);
                }
                QueryNode::Index(index) => {
                    match cur_value {
                        Some(Value::Array(array)) => cur_value = Some(array.get(*index)?),
                        _ => return None,
                    };
                }

            }
        }
        cur_value
    }
}

/*
 *     pub fn get<'a>(&'a self, refs: &'a Vec<Ref>, type_map: &TypeMap) -> Option<&'a Value> {
        let mut it = refs.iter().peekable();
        let mut root = *self.roots.get(0)? as usize;
        if let Some(first) = it.peek() {
            if let Ref::Index(idx) = first {
                root = *self.roots.get(*idx)? as usize;
                it.next();
            }
        }
        let root = self.instances.get(root)?;
        let mut value = None;
        // first ref must be a field name
        if let Some(r) = it.next() {
            if let Ref::Field(name) = r {
                let cur_type = type_map.get_by_hash(root.hash)?;
                let name_hash = murmur3(&name, 0xffffffff);
                let field_idx = cur_type.fields.get_index_of(&name_hash)?;
                value = root.fields.get(field_idx);
            }
        }

        while let Some(r) = it.next() {
            if let Some(val) = value {
                if let Value::Object(idx) = val {
                    if let Ref::Field(name) = r {
                        let cur_instance = self.instances.get(*idx as usize)?;
                        let cur_type = type_map.get_by_hash(cur_instance.hash)?;
                        let name_hash = murmur3(&name, 0xffffffff);
                        let field_idx = cur_type.fields.get_index_of(&name_hash)?;
                        value = cur_instance.fields.get(field_idx);
                    }
                }
                else if let Value::Array(vals) = val {
                    if let Ref::Index(idx) = r {
                        value = vals.get(*idx);
                    }
                } else {
                    break
                }
            } else {
                break
            }
        }
        value
    }

    // var_ref_idx is the index of what ref to variate on (must be an array)
    // it will finish once the target value is reached
    pub fn get_var_index<'a>(&'a self, refs: &'a Vec<Ref>, var_ref_idx: usize, target: &Value, type_map: &TypeMap) -> Option<usize> {
        let mut it = refs.iter().enumerate().peekable();
        let mut root = *self.roots.get(0)? as usize;
        if let Some(first) = it.peek() {
            if let Ref::Index(idx) = first.1 {
                root = *self.roots.get(*idx)? as usize;
                it.next();
            }
        }
        let mut cur_instance = self.instances.get(root)?;
        let mut value = None;
        // first ref must be a field name
        if let Some(r) = it.next() {
            if let Ref::Field(name) = r.1 {
                let name_hash = murmur3(&name, 0xffffffff);
                let cur_type = type_map.get_by_hash(cur_instance.hash)?;
                let field_idx = cur_type.fields.get_index_of(&name_hash)?;
                value = cur_instance.fields.get(field_idx);
            }
        }
        if it.peek()?.0 != var_ref_idx {
            while let Some(r) = it.next() {
                println!("{r:?}");
                if let Some(val) = value {
                    if let Value::Object(idx) = val {
                        if let Ref::Field(name) = r.1 {
                            cur_instance = self.instances.get(*idx as usize)?;
                            let cur_type = type_map.get_by_hash(cur_instance.hash)?;
                            let name_hash = murmur3(&name, 0xffffffff);
                            let field_idx = cur_type.fields.get_index_of(&name_hash)?;
                            value = cur_instance.fields.get(field_idx);
                        }
                    }
                    else if let Value::Array(vals) = val {
                        if let Ref::Index(idx) = r.1 {
                            value = vals.get(*idx);
                        }
                    } else {
                        break
                    }
                } else {
                    break
                }

                if r.0 == var_ref_idx {
                    break;
                }
            }
        }

        if let Some(val) = value {
            if let Value::Array(vals) = val {
                for (i, value) in vals.iter().enumerate() {
                    //println!("arr_index={i:?}, {value:?}, {:?}", cur_type.name);
                    let mut it = it.clone();
                    let mut value = Some(value);
                    while let Some(r) = it.next() {
                        if let Some(val) = value {
                            if let Value::Object(idx) = val {
                                if let Ref::Field(name) = r.1 {
                                    let cur_instance = self.instances.get(*idx as usize)?;
                                    let cur_type = type_map.get_by_hash(cur_instance.hash)?;
                                    let name_hash = murmur3(&name, 0xffffffff);
                                    let field_idx = cur_type.fields.get_index_of(&name_hash)?;
                                    value = cur_instance.fields.get(field_idx);
                                }
                            }
                            else if let Value::Array(vals) = val {
                                if let Ref::Index(idx) = r.1 {
                                    value = vals.get(*idx);
                                }
                            } else {
                                break
                            }
                        } else {
                            break
                        }
                    }
                    if let Some(val) = value {
                        if val.as_i128()? == target.as_i128()? {
                            return Some(i);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn get_with_var<'a>(&'a self, refs: &'a Vec<Ref>, var_idx: usize, var_val: usize, type_map: &TypeMap) -> Option<&'a Value> {
        let mut it = refs.iter().enumerate().peekable();
        let mut root = *self.roots.get(0)? as usize;
        if let Some(first) = it.peek() {
            if let Ref::Index(idx) = first.1 {
                root = *self.roots.get(*idx)? as usize;
                it.next();
            }
        }
        let cur_instance = self.instances.get(root)?;
        let mut value = None;
        // first ref must be a field name
        if let Some(r) = it.next() {
            if let Ref::Field(name) = r.1 {
                let name_hash = murmur3(&name, 0xffffffff);
                let cur_type = type_map.get_by_hash(cur_instance.hash)?;
                let field_idx = cur_type.fields.get_index_of(&name_hash)?;
                value = cur_instance.fields.get(field_idx);
            }
        }

        while let Some(r) = it.next() {
            if r.0 == var_idx {
                if let Some(val) = value {
                    if let Value::Array(vals) = val {
                        value = vals.get(var_val);
                    }
                }
            }
            if let Some(val) = value {
                if let Value::Object(idx) = val {
                    if let Ref::Field(name) = r.1 {
                        let cur_instance = self.instances.get(*idx as usize)?;
                        let cur_type = type_map.get_by_hash(cur_instance.hash)?;
                        let name_hash = murmur3(&name, 0xffffffff);
                        let field_idx = cur_type.fields.get_index_of(&name_hash)?;
                        value = cur_instance.fields.get(field_idx);
                    }
                }
                else if let Value::Array(vals) = val {
                    if let Ref::Index(idx) = r.1 {
                        value = vals.get(*idx);
                    }
                } else {
                    break
                }
            } else {
                break
            }
        }
        value
    }

}
*/
