PORTNAME=	menulibre
PORTVERSION=	2.4.0
PORTREVISION=	2
DISTVERSIONPREFIX=	${PORTNAME}-
CATEGORIES=	x11
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

MAINTAINER=	madpilot@FreeBSD.org
COMMENT=	Advanced menu editor
WWW=		https://bluesabre.org/projects/menulibre/

LICENSE=	GPLv3

BUILD_DEPENDS=	${PYTHON_PKGNAMEPREFIX}python-distutils-extra>=2.38:devel/py-python-distutils-extra@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}psutil>=2.0:sysutils/py-psutil@${PY_FLAVOR}
LIB_DEPENDS=	libgnome-menu-3.so:x11/gnome-menus
RUN_DEPENDS=	xdg-open:devel/xdg-utils \
		pkexec:sysutils/polkit \
		${PYTHON_PKGNAMEPREFIX}psutil>=2.0:sysutils/py-psutil@${PY_FLAVOR}

USES=		gettext gnome python
USE_GNOME=	intltool pygobject3
USE_PYTHON=	distutils

USE_GITHUB=	yes
GH_ACCOUNT=	bluesabre

NO_ARCH=	yes

OPTIONS_DEFINE=	DOCS

.include <bsd.port.mk>
