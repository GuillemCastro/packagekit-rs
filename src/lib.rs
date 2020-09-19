
use packagekit_sys as packagekit;
use std::{ptr, ffi::CString, error, fmt};
use libc::{c_char};

#[derive(Debug, Clone)]
pub struct Package {
    id: String,
    name: String,
    summary: String,
    version: String,
    data: String,
    arch: String,
}

fn string_from_raw(ptr: *const c_char) -> String {
    unsafe {
        if ptr.is_null() {
            return String::new();
        }
        let c_str = CString::from_raw(ptr as *mut c_char);
        return match  c_str.to_str() {
            Ok(s) => s.to_owned(),
            Err(_) => {
                String::new()
            }
        };
    }
}

impl Package {

    pub fn new(id: String, name: String, version: String, summary: String, arch: String, data: String) -> Self {
        return Package{
            id: id,
            name: name,
            summary: summary,
            version: version,
            data: data,
            arch: arch,
        }
    }

    pub fn from_raw(pkg: *mut packagekit::PkPackage) -> Self {
        unsafe {
            let raw_id = packagekit::pk_package_get_id(pkg);
            let id = string_from_raw( raw_id );
            let summary =  string_from_raw( packagekit::pk_package_get_summary(pkg));
            let mut raw_idd: Vec<c_char> = id.clone().into_bytes().iter().map(|b| *b as c_char).collect();
            let raw_id2 = raw_idd.as_mut_ptr();
            let split = packagekit::pk_package_id_split(raw_id2);
            let split_vec = Vec::from_raw_parts(split, 4, 4);
            let name = string_from_raw(split_vec[packagekit::PK_PACKAGE_ID_NAME as usize]);
            let version = string_from_raw(split_vec[packagekit::PK_PACKAGE_ID_VERSION as usize]);
            let data = string_from_raw(split_vec[packagekit::PK_PACKAGE_ID_DATA as usize]);
            let arch = string_from_raw(split_vec[packagekit::PK_PACKAGE_ID_ARCH as usize]);
            Self::new(id, name, version, summary, arch, data)
        }
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn summary(&self) -> &str {
        self.summary.as_str()
    }

    pub fn version(&self) -> &str {
        self.version.as_str()
    }

    pub fn data(&self) -> &str {
        self.data.as_str()
    }

    pub fn arch(&self) -> &str {
        self.arch.as_str()
    }

}

impl PartialEq for Package {

    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id) && self.name.eq(&other.name) 
        && self.summary.eq(&other.summary) && self.version.eq(&other.version)
        && self.data.eq(&other.data) && self.arch.eq(&other.arch)
    }

}

impl Eq for Package {

}

#[derive(Debug)]
pub struct PackageKit {
}

impl PackageKit {

    pub fn new() -> Self {
        return PackageKit {
        }
    }

    pub fn search_package(&self, name: &str) -> Result<Vec<Package>, Error> {
        let mut found_packages: Vec<Package> = Vec::new();
        let bytes = String::from(name).into_bytes();
        let mut cchars: Vec<c_char> = bytes.iter().map(|b| *b as c_char).collect();
        let mut pkgs = vec![cchars.as_mut_ptr()];
        let ppname = pkgs.as_mut_ptr();
        unsafe {
            let pktask = packagekit::pk_task_new();   
            let result = packagekit::pk_task_resolve_sync(
                pktask,
                packagekit::PK_FILTER_ENUM_NOT_INSTALLED as u64,
                ppname,
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                ptr::null_mut()
            );
            let error = packagekit::pk_results_get_error_code(result);
            if !error.is_null() {
                let error_code = packagekit::pk_error_get_code(error);
                let error_msg = packagekit::pk_error_enum_to_string(error_code);
                let msg = string_from_raw(error_msg);
                return Err(Error::new(msg))
            }
            let array = packagekit::pk_results_get_package_array(result);
            let vecc = Vec::from_raw_parts((*array).pdata as *mut *mut packagekit::PkPackage, (*array).len as usize, (*array).len as usize);
            for pkg in vecc {
                let pp = Package::from_raw(pkg);
                found_packages.push( pp );
            }
        }
        Ok(found_packages)
    }

    pub fn install(&self, pkg: &Package) -> Result<(), Error> {
        self.install_packages(&vec![pkg.to_owned()])
    }

    pub fn install_packages(&self, pkgs: &Vec<Package>) -> Result<(), Error> {
        let ids: Vec<String> = pkgs.iter().map(|p| p.id.clone()).collect();
        let native_ids: Vec<CString> = ids.iter().map(
            |id|
            CString::new(id.clone()).unwrap()
        ).collect();
        let mut vec_raw: Vec<*mut c_char> = native_ids.iter().map(
            |id|
            (id.clone()).into_raw()
        ).collect();
        let ptr_pkgs_id_raw = vec_raw.as_mut_ptr();
        unsafe {
            let pktask = packagekit::pk_task_new();
            let result = packagekit::pk_task_install_packages_sync(
                pktask, 
                ptr_pkgs_id_raw, 
                ptr::null_mut(), 
                None, 
                ptr::null_mut(), 
                ptr::null_mut()
            );
            let error = packagekit::pk_results_get_error_code(result);
            if !error.is_null() {
                let error_code = packagekit::pk_error_get_code(error);
                let error_msg = packagekit::pk_error_enum_to_string(error_code);
                let msg = string_from_raw(error_msg);
                return Err(Error::new(msg))
            }
        }
        Ok(())
    }

}
#[derive(Debug)]
pub struct Error {
    msg: String
}

impl Error {

    pub fn new(msg: String) -> Self {
        return Error {
            msg: msg
        }
    }

}

impl error::Error for Error {

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }

}