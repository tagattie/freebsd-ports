# $FreeBSD$

PORTNAME=	caffeine-ng
PORTVERSION=	3.4.2
CATEGORIES=	x11 python
MASTER_SITES=	CHEESESHOP
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

MAINTAINER=	tagattie@yandex.com
COMMENT=	Tray bar app to temporarily inhibit screensaver and sleep mode

LICENSE=	GPLv3+
LICENSE_FILE=	${WRKSRC}/LICENCE

BUILD_DEPENDS=	${PYTHON_PKGNAMEPREFIX}setuptools_scm>0:devel/py-setuptools_scm@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}docopt>=0.6.2:devel/py-docopt@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}ewmh>=0.1.4:devel/py-ewmh@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}xdg>=0.25:devel/py-xdg@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}setproctitle>=1.1.10:devel/py-setproctitle@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}wheel>=0.29.0:devel/py-wheel@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}dbus>0:devel/py-dbus@${PY_FLAVOR}
LIB_DEPENDS=	libappindicator3.so:devel/libappindicator \
		libnotify.so:devel/libnotify
RUN_DEPENDS=	${PYTHON_PKGNAMEPREFIX}docopt>=0.6.2:devel/py-docopt@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}ewmh>=0.1.4:devel/py-ewmh@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}xdg>=0.25:devel/py-xdg@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}setproctitle>=1.1.10:devel/py-setproctitle@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}wheel>=0.29.0:devel/py-wheel@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}dbus>0:devel/py-dbus@${PY_FLAVOR}

USES=		desktop-file-utils gettext-tools gnome python:3.3+
USE_GNOME=	gdkpixbuf2 gtk30 pygobject3
USE_PYTHON=	autoplist distutils

GLIB_SCHEMAS=	net.launchpad.caffeine.gschema.xml
INSTALLS_ICONS=	yes

post-patch:
	${REINPLACE_CMD} -e 's|/usr|${LOCALBASE}|g; s|/etc|${LOCALBASE}/etc|g' \
		${WRKSRC}/setup.py \
		${WRKSRC}/share/applications/caffeine.desktop \
		${WRKSRC}/share/applications/caffeine-preferences.desktop

.include <bsd.port.mk>
