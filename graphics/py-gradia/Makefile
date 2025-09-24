PORTNAME=	gradia
DISTVERSIONPREFIX=	v
DISTVERSION=	1.10.1
CATEGORIES=	graphics
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

MAINTAINER=	tagattie@FreeBSD.org
COMMENT=	Make your screenshots ready for all
WWW=		https://gradia.alexandervanhee.be/

LICENSE=		GPLv3 OFL11
LICENSE_COMB=		multi
LICENSE_FILE_GPLv3=	${WRKSRC}/COPYING
LICENSE_FILE_OFL11=	${WRKSRC}/data/fonts/LICENSE-OFL.txt

BUILD_DEPENDS=	appstreamcli:devel/appstream \
		blueprint-compiler:devel/blueprint-compiler
RUN_DEPENDS=	xdg-desktop-portal>0:deskutils/xdg-desktop-portal \
		${PYTHON_PKGNAMEPREFIX}pillow>0:graphics/py-pillow@${PY_FLAVOR} \
		webp-pixbuf-loader>0:graphics/webp-pixbuf-loader

USES=		desktop-file-utils gettext-tools gnome meson pkgconfig python \
		shebangfix

USE_GITHUB=	yes
GH_ACCOUNT=	AlexanderVanhee
GH_PROJECT=	Gradia

USE_GNOME=	glib20 gtk40 gtksourceview5 libadwaita pygobject3
GLIB_SCHEMAS=	be.alexandervanhee.gradia.gschema.xml

SHEBANG_FILES=	gradia/gradia.in

BINARY_ALIAS=	python3=${PYTHON_CMD}

PORTDOCS=	README.md TRANSLATING.md

OPTIONS_DEFINE=	DOCS

post-install:
	@${RM} ${STAGEDIR}${PREFIX}/share/fonts/LICENSE-OFL.txt

post-install-DOCS-on:
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_MAN} ${PORTDOCS:S|^|${WRKSRC}/|} ${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>
