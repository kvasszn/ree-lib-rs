use std::{io::{self, Read, Write}};
use bytemuck::Pod;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use crate::{types::*, util::{read_pod, write_pod}};

pub trait RszType: Sized {
    fn read_rsz<R: Read>(r: &mut R) -> io::Result<Self>;
    fn write_rsz<W: Write>(&self, w: &mut W) -> io::Result<()>;
}

impl<T: Pod> RszType for T {
    fn read_rsz<R: Read>(r: &mut R) -> io::Result<Self> {
        read_pod::<T, R>(r)
    }

    fn write_rsz<W: Write>(&self, w: &mut W) -> io::Result<()> {
        write_pod::<T, W>(w, self)
    }
}

impl RszType for Data {
    fn read_rsz<R: Read>(r: &mut R) -> io::Result<Self> {
        let len = r.read_u32::<LE>()?;
        let mut v = vec![0u8; len as usize];
        r.read_exact(&mut v)?;
        Ok(Self(v))
    }

    fn write_rsz<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_u32::<LE>(self.0.len() as u32)?;
        w.write_all(&self.0)?;
        Ok(())
    }
}

impl RszType for StringU16 {
    fn read_rsz<R: Read>(r: &mut R) -> io::Result<Self> {
        let len = r.read_u32::<LE>()?;
        let mut s = vec![0u16; len as usize];
        r.read_u16_into::<LE>(&mut s)?;
        Ok(Self(s))
    }

    fn write_rsz<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_u32::<LE>(self.0.len() as u32)?;
        for &c in &self.0 {
            w.write_u16::<LE>(c)?;
        }
        Ok(())
    }
}

impl RszType for StringU16C {
    fn read_rsz<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut s = Vec::new();
        loop {
            let c = r.read_u16::<LE>()?;
            if c == 0 { break; }
            s.push(c);
        }
        Ok(Self(s))
    }

    fn write_rsz<W: Write>(&self, w: &mut W) -> io::Result<()> {
        for &c in &self.0 {
            w.write_u16::<LE>(c)?;
        }
        w.write_u16::<LE>(0)?;
        Ok(())
    }
}
