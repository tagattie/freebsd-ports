# $FreeBSD$

PORTNAME=	materia-gtk-theme
DISTVERSIONPREFIX=	v
DISTVERSION=	20190201
CATEGORIES=	x11-themes

MAINTAINER=	tagattie@yandex.com
COMMENT=	Material Design theme for GNOME/GTK+ based desktop environments

LICENSE=	GPLv2
LICENSE_FILE=	${WRKSRC}/COPYING

BUILD_DEPENDS=	bash:shells/bash \
		gnome-shell:x11/gnome-shell
RUN_DEPENDS=	gtk-murrine-engine>0:x11-themes/gtk-murrine-engine \
		gnome-themes-extra>0:x11-themes/gnome-themes-extra

USES=		shebangfix

USE_GITHUB=	yes
GH_ACCOUNT=	nana-4
GH_PROJECT=	materia-theme

SHEBANG_FILES=	install.sh

OPTIONS_DEFINE=	DOCS

NO_ARCH=	yes
NO_BUILD=	yes

PORTDOCS=	README.md README-gnome-shell.md

do-install:
	${WRKSRC}/install.sh --dest ${STAGEDIR}${PREFIX}/share/themes

post-install:
	${RM} ${STAGEDIR}${PREFIX}/share/themes/*/COPYING

post-install-DOCS-on:
	${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_MAN} ${WRKSRC}/README.md ${STAGEDIR}${DOCSDIR}
	${INSTALL_MAN} ${WRKSRC}/src/gnome-shell/README.md ${STAGEDIR}${DOCSDIR}/README-gnome-shell.md

.include <bsd.port.mk>
