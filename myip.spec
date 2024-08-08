Name: myip
Version: 0.1.1
Release: 1
License: MIT
Summary: Prints public ip
Url: https://github.com/ToroidalFox/myip
Source0: %{url}/archive/refs/tags/v%{version}.tar.gz

BuildRequires: cargo
BuildRequires: pkgconf
BuildRequires: openssl-devel
BuildRequires: perl

Requires: openssl-libs

%description

Prints public ip

%prep
%autosetup

%build
cargo build --release

%install
install -Dm755 target/release/%{name} -t %{buildroot}%{_bindir}

%files
%license LICENSE*

%{_bindir}/%{name}
