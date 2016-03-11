pkg_name=mongodb
pkg_version=3.0.10
pkg_origin=chef
pkg_license=('mongodb')
pkg_source=https://github.com/mongodb/mongo/archive/r${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=459ba011980f5ea5eca4d5e02c85fc73482b02c9f5fe80abf089d6ea5dc0e730
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc chef/zlib)
pkg_build_deps=(chef/python2 chef/coreutils chef/scons chef/binutils)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_filename=r${pkg_version}.tar.gz
pkg_dirname=mongo-r${pkg_version}

do_build() {
  scons mongod
  return 0
}

do_install() {
  return 0
}
