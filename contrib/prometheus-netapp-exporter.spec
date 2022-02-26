Name:		prometheus-netapp-exporter
Version:	1.0.1
Release:	1%{?dist}
Summary:	Export NetApp (C-mode) metrics from the OnTap REST API to Prometheus

Group:		System Environment/Daemons
License:	GPLv3
URL:		https://git.ypbind.de/cgit/prometheus-netapp-exporter
Source0:	https://git.ypbind.de/cgit/prometheus-netapp-exporter/snapshot/prometheus-netapp-exporter-1.0.1.tar.gz

%define _build_id_links none
%description
In an enterprise environment, NetApp storage is widely used.
Starting with OnTap 9.6, a REST API is provided (the old XML
RPC API is still present at the moment).

This tool fetches statistics from the REST API of the NetApp
fileserver and export the data for Prometheus.

%global debug_package %{nil}
%prep
%setup -q

%build
make build strip


%install
make install DESTDIR=%{buildroot}
mkdir -m 0755 -p %{buildroot}/%{_unitdir}/
mv %{buildroot}/lib/systemd/system/prometheus-netapp-exporter.service %{buildroot}/%{_unitdir}/prometheus-netapp-exporter.service
rm -rf %{buildroot}/lib
rm -f {buildroot}/usr/lib/.build-id

%files
%defattr(-,root,root,-)
%{_sbindir}/prometheus-netapp-exporter
%{_unitdir}/prometheus-netapp-exporter.service
%{_mandir}/man1/%{name}.1.gz

%changelog
* Sat Feb 26 2022 Andreas Maus <maus@centos8build.badphish.ypbind.de> - 1.0.1-1.el8
- fixing usage string
- man page has been added

* Sat Feb 26 2022 Andreas Maus <maus@centos8build.badphish.ypbind.de> - 1.0.0-1.el8
- First version
