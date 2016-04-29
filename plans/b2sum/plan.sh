pkg_origin=core
pkg_name=b2sum
pkg_version=master
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('CC0' 'OpenSSL' 'Apache2.0')
pkg_source=nosuchfile.tgz
pkg_shasum=sha256sum
pkg_deps=(core/glibc core/gmp)
pkg_build_deps=(core/coreutils core/cacerts core/make core/gcc core/git)
pkg_bin_dirs=(bin)
#pkg_include_dirs=(include)
#pkg_lib_dirs=(lib)

do_download() {
  cd $HAB_CACHE_SRC_PATH
  export GIT_SSL_CAINFO="$(pkg_path_for core/cacerts)/ssl/certs/cacert.pem"
  git clone https://github.com/BLAKE2/BLAKE2.git
  pushd BLAKE2
  git checkout $pkg_version
  popd
  #tar -cjvf $HAB_CACHE_SRC_PATH/${pkg_name}-${pkg_version}.tar.bz2 \
  #    --transform "s,^\./chef,chef${pkg_version}," ./chef \
  #    --exclude chef/.git --exclude chef/spec
  #pkg_shasum=$(trim $(sha256sum $HAB_CACHE_SRC_PATH/${pkg_filename} | cut -d " " -f 1))
}

do_verify() {
    printf "DO_VERIFY"
    return 0
}

do_unpack() {
    return 0
}

do_build() {
    cd $HAB_CACHE_SRC_PATH/BLAKE2/b2sum && make
    return 0
}

do_install() {
    cd $HAB_CACHE_SRC_PATH/BLAKE2/b2sum
    cp b2sum $pkg_prefix/b2sum
    return 0
}

