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
	test -d $(DESTDIR)/lib/systemd/system/ || mkdir -m 0755 -p $(DESTDIR)/lib/systemd/system/
	test -d $(DESTDIR)/usr/share/man/man1/ || mkdir -m 0755 -p $(DESTDIR)/usr/share/man/man1/
	install -m 0755 target/release/$(BINARY) $(DESTDIR)/usr/sbin
	install -m 0644 systemd/prometheus-netapp-exporter.service $(DESTDIR)/lib/systemd/system/
	install -m 0644 doc/man/man1/prometheus-netapp-exporter.1 $(DESTDIR)/usr/share/man/man1/
	gzip -9v $(DESTDIR)/usr/share/man/man1/prometheus-netapp-exporter.1

uninstall:
	/bin/rm -f $(DESTDIR)/usr/sbin/$(BINARY) $(DESTDIR)/lib/systemd/system/prometheus-netapp-exporter.service

