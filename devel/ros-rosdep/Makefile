PORTNAME=	ros-rosdep
DISTVERSION=	0.21.0
PORTREVISION=	2
CATEGORIES=	devel
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

MAINTAINER=	trueos@norwegianrockcat.com
COMMENT=	Multi-package manager system dependency tool for ROS
WWW=		https://docs.ros.org/independent/api/rosdep/html/

LICENSE=	BSD3CLAUSE
LICENSE_FILE=	${WRKSRC}/LICENSE

RUN_DEPENDS=	${PYTHON_PKGNAMEPREFIX}ros-catkin_pkg>=0.4.0:devel/ros-catkin_pkg@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}ros-rospkg>=1:devel/ros-rospkg@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}ros-rosdistro>=0:devel/ros-rosdistro@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}pyyaml>=3:devel/py-pyyaml@${PY_FLAVOR}

USES=		python

USE_PYTHON=	autoplist distutils

USE_GITHUB=	yes
GH_ACCOUNT=	ros-infrastructure
GH_PROJECT=	rosdep

.include <bsd.port.mk>
