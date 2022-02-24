.PHONY: all build strip install clean
BINARY=prometheus-netapp-exporter

all: build strip install

build:
	env PATH=${PATH}:${HOME}/.cargo/bin cargo build --release

strip: build
	strip --strip-all target/release/$(BINARY)

clean:
	env PATH=${PATH}:${HOME}/.cargo/bin cargo clean

install: strip
	test -d $(DESTDIR)/usr/sbin || mkdir -m 0755 -p $(DESTDIR)/usr/sbin
	install -m 0755 target/release/$(BINARY) $(DESTDIR)/usr/sbin
	install -m 0644 systemd/prometheus-netapp-exporter.service $(DESTDIR)/lib/systemd/system

uninstall:
	/bin/rm -f $(DESTDIR)/usr/sbin/$(BINARY) $(DESTDIR)/lib/systemd/system/prometheus-netapp-exporter.service

