PORTNAME=	librdkafka
DISTVERSIONPREFIX=	v
DISTVERSION=	2.8.0
CATEGORIES=	net

MAINTAINER=	pizzamig@FreeBSD.org
COMMENT=	Apache Kafka C/C++ library
WWW=		https://github.com/edenhill/librdkafka/

LICENSE=	BSD2CLAUSE
LICENSE_FILE=	${WRKSRC}/LICENSE

BROKEN_mips=		fails to build: undefined reference to __sync_add_and_fetch_4
BROKEN_mips64=		fails to build: undefined reference to __sync_add_and_fetch_4

BUILD_DEPENDS=	bash:shells/bash \
	${LOCALBASE}/include/rapidjson/schema.h:devel/rapidjson
LIB_DEPENDS=	libcurl.so:ftp/curl \
	liblz4.so:archivers/liblz4 \
	libzstd.so:archivers/zstd

USES=		compiler:c11 gmake localbase:ldflags ssl
USE_GITHUB=	yes

GH_ACCOUNT=	edenhill

GNU_CONFIGURE=	yes
MAKE_ARGS=	pkgconfigdir=${PREFIX}/libdata/pkgconfig
CONFIGURE_ARGS+=	--disable-sasl \
					--enable-ssl \
					--enable-strip
LDFLAGS+=      -Wl,--undefined-version

USE_LDCONFIG=	yes

.include <bsd.port.mk>
