PORTNAME=	picture-of-the-day
DISTVERSIONPREFIX=	v
DISTVERSION=	1.8.1
CATEGORIES=	x11
MASTER_SITES=	https://codeberg.org/swsnr/${PORTNAME}/archive/${DISTVERSIONFULL}${EXTRACT_SUFX}?dummy=/
DISTFILES=	${DISTNAME}${EXTRACT_SUFX}

MAINTAINER=	tagattie@FreeBSD.org
COMMENT=	Your daily GNOME wallpaper
WWW=		https://codeberg.org/swsnr/picture-of-the-day

LICENSE=	EUPL12
LICENSE_FILE=	${WRKSRC}/LICENSE

BUILD_DEPENDS=	just:deskutils/just \
		blueprint-compiler:devel/blueprint-compiler \
		gcp:sysutils/coreutils \
		gsed:textproc/gsed
LIB_DEPENDS=	libsoup-3.0.so:devel/libsoup3

USES=		cargo gettext-tools gnome

USE_GNOME=	gtk40 libadwaita
GLIB_SCHEMAS=	de.swsnr.pictureoftheday.Devel.gschema.xml

BINARY_ALIAS=	cp=gcp \
		install=ginstall \
		sed=gsed

WRKSRC=		${WRKDIR}/${PORTNAME}

PORTDOCS=	README.md

OPTIONS_DEFINE=	DOCS

post-configure:
	@cd ${WRKSRC} && ${SETENV} ${CONFIGURE_ENV} just version= compile

do-install:
	@cd ${WRKSRC} && ${SETENV} ${MAKE_ENV} ${CARGO_ENV} \
		${CARGO} install \
			--frozen --no-track --path "" \
			--root "${STAGEDIR}${PREFIX}" ${CARGO_INSTALL_ARGS}
	@cd ${WRKSRC} && ${SETENV} ${MAKE_ENV} \
		just version= DESTPREFIX=${STAGEDIR}${PREFIX} install

do-install-DOCS-on:
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_MAN} ${PORTDOCS:S|^|${WRKSRC}/|} ${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>
