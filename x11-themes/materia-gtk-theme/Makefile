# $FreeBSD$

PORTNAME=	materia-gtk-theme
DISTVERSIONPREFIX=	v
DISTVERSION=	20190201
CATEGORIES=	x11-themes

MAINTAINER=	tagattie@yandex.com
COMMENT=	Material Design theme for GNOME/GTK+ based desktop environments

LICENSE=	GPLv2
LICENSE_FILE=	${WRKSRC}/COPYING

BUILD_DEPENDS=	bash:shells/bash
RUN_DEPENDS=	gtk-murrine-engine>0:x11-themes/gtk-murrine-engine \
		gnome-themes-extra>0:x11-themes/gnome-themes-extra

USES=		shebangfix

USE_GITHUB=	yes
GH_ACCOUNT=	nana-4
GH_PROJECT=	materia-theme

SHEBANG_FILES=	install.sh

NO_ARCH=	yes

PORTDOCS=	README.md

OPTIONS_DEFINE=	DOCS GNOMESHELL
OPTIONS_DEFAULT=	GNOMESHELL
OPTIONS_SUB=	yes

GNOMESHELL_DESC=	Add Gnome-Shell theme version
GNOMESHELL_BUILD_DEPENDS=	gnome-shell:x11/gnome-shell

.include <bsd.port.options.mk>

.if ${PORT_OPTIONS:MGNOMESHELL}
PORTDOCS+=	src/gnome-shell/README.md
.endif

do-build:
	${MKDIR} ${WRKDIR}/build
	${WRKSRC}/install.sh --dest ${WRKDIR}/build

post-build:
	${RM} ${WRKDIR}/build/*/COPYING
	${RM} ${WRKDIR}/build/*/gnome-shell/README.md

post-build-GNOMESHELL-off:
	${RM} -R ${WRKDIR}/build/*/gnome-shell

do-install:
	${MKDIR} ${STAGEDIR}${PREFIX}/share/themes
	(cd ${WRKDIR}/build && \
		${COPYTREE_SHARE} . ${STAGEDIR}${PREFIX}/share/themes)

post-install-DOCS-on:
	${MKDIR} ${STAGEDIR}${DOCSDIR}
.for d in ${PORTDOCS}
	(cd ${WRKSRC} && \
		${COPYTREE_SHARE} ${d} ${STAGEDIR}${DOCSDIR})
.endfor

.include <bsd.port.mk>
