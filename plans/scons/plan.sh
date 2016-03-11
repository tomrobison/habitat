pkg_name=scons
pkg_version=2.4.1
pkg_origin=chef
pkg_license=('MIT')
pkg_source=http://downloads.sourceforge.net/sourceforge/scons/scons-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=8fc4f42928c69bcbb33e1be94b646f2c700b659693fabc778c192d4d22f753a7
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc chef/python2)
pkg_build_deps=(chef/python2 chef/coreutils)
pkg_lib_dirs=(lib)
pkg_binary_path=(bin)
pkg_include_dirs=(include)

do_build() {
  return 0
}

do_install() {
  python setup.py install --prefix=$pkg_path
  fix_interpreter "${PREFIX}/bin/*" chef/coreutils bin/env
  return 0
}

