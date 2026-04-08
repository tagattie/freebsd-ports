PORTNAME=	nbd
DISTVERSION=	3.25
PORTEPOCH=	1
CATEGORIES=	net
MASTER_SITES=	https://github.com/NetworkBlockDevice/${PORTNAME}/releases/download/${PORTNAME}-${DISTVERSION}/
PKGNAMESUFFIX=	-server

MAINTAINER=	bofh@FreeBSD.org
COMMENT=	Server for the Linux network block device (nbd)
WWW=		https://github.com/NetworkBlockDevice/nbd

LICENSE=	GPLv2
LICENSE_FILE=	${WRKSRC}/COPYING

USES=		bison:wrapper cpe gnome pkgconfig tar:xz
CPE_VENDOR=	wouter_verhelst
USE_GNOME=	glib20

GNU_CONFIGURE=	yes
CONFIGURE_ARGS=	--enable-lfs --enable-syslog

TEST_TARGET=	check

PLIST_FILES=	bin/nbd-server bin/nbd-trdump bin/nbd-trplay \
		share/man/man1/nbd-server.1.gz \
		share/man/man1/nbd-trdump.1.gz \
		share/man/man1/nbd-trplay.1.gz \
		share/man/man5/nbd-server.5.gz \
		share/man/man5/nbdtab.5.gz \
		share/man/man8/nbd-client.8.gz

OPTIONS_DEFINE=	GNUTLS

GNUTLS_BUILD_DEPENDS=	gnutls>0:security/gnutls
GNUTLS_RUN_DEPENDS=	gnutls>0:security/gnutls
GNUTLS_CONFIGURE_WITH=	gnutls

.include <bsd.port.mk>
