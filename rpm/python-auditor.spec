Name:           python-auditor
Version:        0.7.0
Release:        1%{?dist}
# Fill in the actual package summary to submit package to Fedora
Summary:        Python interface to AuditorClient

# Check if the automatically generated License and its spelling is correct for Fedora
# https://docs.fedoraproject.org/en-US/packaging-guidelines/LicensingGuidelines/
License:        MIT OR Apache-2.0
URL:            https://alu-schumacher.github.io/AUDITOR/
Source:         %{pypi_source python_auditor}

BuildRequires:  python3-devel
BuildRequires:  gcc
%global debug_package %{nil}
# Fill in the actual package description to submit package to Fedora
%global _description %{expand:
This is package 'python-auditor' generated automatically by pyp2spec.}

%description %_description

%package -n     python3-python-auditor
Summary:        %{summary}

%description -n python3-python-auditor %_description


%prep
pip install --user --upgrade pip
pip install --user maturin
%autosetup -p1 -n python_auditor-%{version}

# %generate_buildrequires
# %pyproject_buildrequires


%build
export PATH=$PATH:$HOME/.local/bin
%pyproject_wheel


%install
%pyproject_install
# Add top-level Python module names here as arguments, you can use globs
%pyproject_save_files pyauditor


%check
%pyproject_check_import


%files -n python3-python-auditor -f %{pyproject_files}


%changelog
%autochangelog
