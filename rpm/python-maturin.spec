Name:           python-maturin
Version:        1.8.2
Release:        %autorelease
# Fill in the actual package summary to submit package to Fedora
Summary:        Build and publish crates with pyo3, cffi and uniffi bindings as well as rust binaries as python packages

# Check if the automatically generated License and its spelling is correct for Fedora
# https://docs.fedoraproject.org/en-US/packaging-guidelines/LicensingGuidelines/
License:        MIT OR Apache-2.0
URL:            https://github.com/pyo3/maturin
Source:         %{pypi_source maturin}

BuildArch:      noarch
BuildRequires:  python3-devel


# Fill in the actual package description to submit package to Fedora
%global _description %{expand:
This is package 'maturin' generated automatically by pyp2spec.}

%description %_description

%package -n     python3-maturin
Summary:        %{summary}

%description -n python3-maturin %_description

%prep
%autosetup -p1 -n maturin-%{version}

%generate_buildrequires
%pyproject_buildrequires


%build
%pyproject_wheel


%install
%pyproject_install
# Add top-level Python module names here as arguments, you can use globs
%pyproject_save_files maturin


%check
%pyproject_check_import


%files -n python3-maturin -f %{pyproject_files}


%changelog
%autochangelog
