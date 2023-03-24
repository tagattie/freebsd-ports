PORTNAME=	bitwarden-cli
DISTVERSIONPREFIX=	cli-v
DISTVERSION=	2023.3.0
CATEGORIES=	security
MASTER_SITES=	https://nodejs.org/dist/v${PKG_NODE_VER}/:node
DISTFILES=	node-v${PKG_NODE_VER}${EXTRACT_SUFX}:node \
		${PREFETCH_FILE}:prefetch

MAINTAINER=	tagattie@FreeBSD.org
COMMENT=	Bitwarden client command-line interface
WWW=		https://bitwarden.com

LICENSE=	BITWARDEN GPLv3
LICENSE_COMB=	multi
LICENSE_NAME_BITWARDEN=	BITWARDEN LICENSE AGREEMENT
LICENSE_FILE_BITWARDEN=	${WRKSRC}/LICENSE_BITWARDEN.txt
LICENSE_FILE_GPLv3=	${WRKSRC}/LICENSE_GPL.txt
LICENSE_PERMS_BITWARDEN=none

ONLY_FOR_ARCHS=	aarch64 amd64 i386

FETCH_DEPENDS=	npm:www/npm${NODEJS_SUFFIX}
BUILD_DEPENDS=	npm:www/npm${NODEJS_SUFFIX}
LIB_DEPENDS=	libbrotlidec.so:archivers/brotli \
		libicui18n.so:devel/icu \
		libuv.so:devel/libuv \
		libcares.so:dns/c-ares \
		libnghttp2.so:www/libnghttp2
RUN_DEPENDS=	xdg-open:devel/xdg-utils \
		ca_root_nss>0:security/ca_root_nss
TEST_DEPENDS=	npm:www/npm${NODEJS_SUFFIX}

USES=		gmake localbase:ldflags nodejs:16,build pkgconfig \
		python:build ssl

USE_GITHUB=	yes
GH_ACCOUNT=	bitwarden
GH_PROJECT=	clients

BUILD_WRKSRC=	${WRKSRC}/apps/cli

PLIST_FILES=	bin/bw

PORTDOCS=	CONTRIBUTING.md README.md SECURITY.md

OPTIONS_DEFINE=	DOCS

PREFETCH_FILE=	${PKGNAME}-node-modules${EXTRACT_SUFX}
PREFETCH_TIMESTAMP=	1679612370
PKG_NODE_VER=	16.16.0
PKG_FETCH_VER=	3.4
PKG_NODE_CONFIGURE_ARGS=--openssl-use-def-ca-store \
			--shared-brotli \
			--shared-cares \
			--shared-libuv \
			--shared-nghttp2 \
			--shared-openssl \
			--shared-zlib \
			--with-intl=system-icu
NODE_ARCH=	${ARCH:S/aarch64/arm64/:S/amd64/x64/:S/i386/ia32/}

pre-fetch:
	if [ ! -f ${DISTDIR}/${PREFETCH_FILE} ]; then \
		${MKDIR} ${WRKDIR}/npm-cache; \
		${CP} ${FILESDIR}/package.json ${FILESDIR}/package-lock.json ${WRKDIR}/npm-cache; \
		cd ${WRKDIR}/npm-cache && \
		${SETENV} HOME=${WRKDIR} npm ci --ignore-scripts --no-progress; \
		${RM} package.json package-lock.json; \
		${FIND} ${WRKDIR}/npm-cache -type d -exec ${CHMOD} 755 {} ';'; \
		cd ${WRKDIR} && \
		${MTREE_CMD} -cbnSp npm-cache | ${MTREE_CMD} -C | ${SED} \
			-e 's:time=[0-9.]*:time=${PREFETCH_TIMESTAMP}.000000000:' \
			-e 's:\([gu]id\)=[0-9]*:\1=0:g' \
			-e 's:flags=.*:flags=none:' \
			-e 's:^\.:./npm-cache:' > npm-cache.mtree; \
		${TAR} -cz --options 'gzip:!timestamp' \
			-f ${DISTDIR}/${PREFETCH_FILE} \
			@npm-cache.mtree; \
		${RM} -r ${WRKDIR}; \
	fi

post-extract:
.for f in package.json package-lock.json
	${MV} -f ${WRKSRC}/${f} ${WRKSRC}/${f}.bak
	${CP} ${FILESDIR}/${f} ${WRKSRC}
.endfor
	${MV} ${WRKDIR}/npm-cache/node_modules ${WRKSRC}

post-patch:
	# apply FreeBSD patches for node
	for p in ${.CURDIR}/../../www/node${NODEJS_VERSION}/files/patch-*; do \
		${PATCH} -s -p0 -d ${WRKDIR}/node-v${PKG_NODE_VER} < $${p}; \
	done
	# apply node patch from pkg-fetch
	${PATCH} -s -p1 -d ${WRKDIR}/node-v${PKG_NODE_VER} < \
		${WRKSRC}/node_modules/pkg-fetch/patches/node.v${PKG_NODE_VER}.cpp.patch

pre-build:
	# build patched node for pkg
	cd ${WRKDIR}/node-v${PKG_NODE_VER} && \
		${SETENV} ${CONFIGURE_ENV} CC=${CC} CXX=${CXX} ./configure ${PKG_NODE_CONFIGURE_ARGS} && \
		${SETENV} ${MAKE_ENV} ${MAKE_CMD} -j ${MAKE_JOBS_NUMBER}
	${MKDIR} ${WRKDIR}/.pkg-cache/v${PKG_FETCH_VER}
	${MV} ${WRKDIR}/node-v${PKG_NODE_VER}/out/Release/node \
		${WRKDIR}/.pkg-cache/v${PKG_FETCH_VER}/built-v${PKG_NODE_VER}-freebsd-${NODE_ARCH}
	${STRIP_CMD} ${WRKDIR}/.pkg-cache/v${PKG_FETCH_VER}/built-v${PKG_NODE_VER}-freebsd-${NODE_ARCH}
	# rebuild node modules against patched node
	cd ${WRKSRC} && ${SETENV} ${MAKE_ENV} ELECTRON_SKIP_BINARY_DOWNLOAD=1 \
		npm rebuild --nodedir=${WRKDIR}/node-v${PKG_NODE_VER} --verbose

do-build:
	cd ${BUILD_WRKSRC} && ${SETENV} ${MAKE_ENV} \
		npm run build:prod
	cd ${BUILD_WRKSRC} && ${SETENV} ${MAKE_ENV} \
		npx pkg . --targets node${NODEJS_VERSION}-freebsd-${NODE_ARCH} --output ./dist/bw

do-install:
	# don't strip executable since it causes error
	${INSTALL_KLD} ${BUILD_WRKSRC}/dist/bw ${STAGEDIR}${PREFIX}/bin

do-install-DOCS-on:
	${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_MAN} ${PORTDOCS:S|^|${WRKSRC}/|} ${STAGEDIR}${DOCSDIR}

do-test:
	cd ${BUILD_WRKSRC} && ${SETENV} ${TEST_ENV} npm run test

.include <bsd.port.mk>
