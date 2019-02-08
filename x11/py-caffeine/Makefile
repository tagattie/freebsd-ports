# $FreeBSD$

PORTNAME=	caffeine
PORTVERSION=	2.9.5
CATEGORIES=	x11 python
MASTER_SITES=	https://launchpad.net/~caffeine-developers/+archive/ubuntu/ppa/+sourcefiles/${PORTNAME}/${PORTVERSION}/
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}
DISTNAME=	${PORTNAME}_${PORTVERSION}

MAINTAINER=	tagattie@yandex.com
COMMENT=	Temporarily deactivate the screensaver and sleep mode

LICENSE=	GPLv3+ LGPL3+
LICENSE_COMB=	multi
LICENSE_FILE_GPLv3+ = ${WRKSRC}/COPYING
LICENSE_FILE_LGPL3+ = ${WRKSRC}/COPYING.LESSER

LIB_DEPENDS=	libappindicator3.so:devel/libappindicator
RUN_DEPENDS=	${PYTHON_PKGNAMEPREFIX}xlib>0:x11-toolkits/py-xlib@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}ewmh>0:devel/py-ewmh@${PY_FLAVOR} \
		xdg-screensaver:devel/xdg-utils

USES=		gettext-tools gnome python:3.5+ shebangfix
USE_GNOME=	intltool gtk30 pygobject3
USE_PYTHON=	autoplist concurrent distutils

WRKSRC=		${WRKDIR}/${PORTNAME}-${PORTVERSION}

SHEBANG_FILES=	setup.py compile_translations.py \
		caffeinate caffeine caffeine-indicator

NO_ARCH=	yes

INSTALLS_ICONS=	yes

REINPLACE_ARGS=	-i ""

post-extract:
	${MV} ${WRKSRC}/share/man ${WRKSRC}
	(cd ${WRKSRC}/man/man1 && ${GUNZIP_CMD} *.gz)

post-patch:
	${REINPLACE_CMD} -e 's|"/"|"${LOCALBASE}/"|' ${WRKSRC}/setup.py
	${REINPLACE_CMD} -e 's|Exec=/usr|Exec=${LOCALBASE}|' \
		${WRKSRC}/etc/xdg/autostart/*.desktop \
		${WRKSRC}/share/applications/*.desktop

.include <bsd.port.mk>
