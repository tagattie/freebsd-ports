# Provide support for NodeJS
#
# Feature:      nodejs
# Usage:        USES=nodejs or USES=nodejs:args
# Valid ARGS:	build and/or run <version>
# version:      lts, current, 10, 14, 16, 17
# Default is:   build,run
# Note:			if you define a version, you must provide run and/or build
#
# Variables, which can be set by a port
#
# USE_NODEJS:	Indicates a NodeJS package manager the port uses.
#
#	Supported package managers are:
#	npm[:ARGS]	The port uses NPM as the package manager.
#	yarn[:ARGS]	The port uses Yarn as the package manager.
#
#	Valid ARGS:	fetch, extract, patch, build, run, test
#		fetch:		Indicates the package manager is needed at
#				fetch time and adds it to FETCH_DEPENDS.
#		extract:	Indicates the package manager is needed at
#				extract time and adds it to EXTRACT_DEPENDS.
#		patch:		Indicates the package manager is needed at
#				patch time and adds it to PATCH_DEPENDS.
#		build:		Indicates the package manager is needed at
#				build time and adds it to BUILD_DEPENDS.
#		run:		Indicates the package manager is needed at
#				run time and adds it to RUN_DEPENDS.
#		test:		Indicates the package manager is needed at
#				test time and adds it to TEST_DEPENDS.
#		Note: If the port specifies none of them, we assume the port
#		requires the package manager at build time only.
#
# Exported variables:
#
# NODEJS_PACKAGE_MANAGER:	The chosen NodeJS package manager.
#
# MAINTAINER: bhughes@FreeBSD.org

.if !defined(_INCLUDE_USES_NODEJS_MK)
_INCLUDE_USES_NODEJS_MK=	yes

_VALID_NODEJS_VERSION=	10 14 16 17 lts current
_NODEJS_VERSION_SUFFIX=	${NODEJS_DEFAULT}

.if ! ${_VALID_NODEJS_VERSION:M${_NODEJS_VERSION_SUFFIX}}
IGNORE=	Invalid nodejs default version ${_NODEJS_VERSION_SUFFIX}; valid versions are ${_VALID_NODEJS_VERSION}
.endif

.if empty(nodejs_ARGS)
nodejs_ARGS=	build,run
.endif

. if ${nodejs_ARGS:M10}
_NODEJS_VERSION_SUFFIX=	10
. elif ${nodejs_ARGS:M14}
_NODEJS_VERSION_SUFFIX=	14
. elif ${nodejs_ARGS:M16}
_NODEJS_VERSION_SUFFIX=	16
. elif ${nodejs_ARGS:Mlts}
_NODEJS_VERSION_SUFFIX=	lts
. elif ${nodejs_ARGS:M17}
_NODEJS_VERSION_SUFFIX=	17
. elif ${nodejs_ARGS:Mcurrent}
_NODEJS_VERSION_SUFFIX=	current
. elif defined(NODEJS_DEFAULT)
. endif

# The nodejs 17 version is named www/node
. if ${_NODEJS_VERSION_SUFFIX:Mcurrent}
_NODEJS_VERSION_SUFFIX=
. endif
. if ${_NODEJS_VERSION_SUFFIX:M17}
_NODEJS_VERSION_SUFFIX=
. endif
# The nodejs LTS is version 16
. if ${_NODEJS_VERSION_SUFFIX:Mlts}
_NODEJS_VERSION_SUFFIX=	16
. endif

. if ${nodejs_ARGS:M*run*}
RUN_DEPENDS+=	node:www/node${_NODEJS_VERSION_SUFFIX}
. endif
. if ${nodejs_ARGS:M*build*}
BUILD_DEPENDS+=	node:www/node${_NODEJS_VERSION_SUFFIX}
. endif

.if defined(USE_NODEJS)
_VALID_NODEJS_FEATURE=	npm yarn

_NODEJS_FEATURE=	${USE_NODEJS:C/:.*//}
_NODEJS_FEATURE_ARGS=	${USE_NODEJS:C/^[^:]*(:|\$)//}

. if !${_VALID_NODEJS_FEATURE:M${_NODEJS_FEATURE}}
IGNORE=	specifies unknown USE_NODEJS feature: ${_NODEJS_FEATURE}
. endif
. if empty(_NODEJS_FEATURE_ARGS)
_NODEJS_FEATURE_ARGS=	build
. endif

. if empty(_NODEJS_VERSION_SUFFIX)
_NODEJS_FEATURE_DEP=	www/${_NODEJS_FEATURE}>0:www/${_NODEJS_FEATURE}
. else
_NODEJS_FEATURE_DEP=	www/${_NODEJS_FEATURE}-node${_NODEJS_VERSION_SUFFIX}>0:www/${_NODEJS_FEATURE}-node${_NODEJS_VERSION_SUFFIX}
. endif

. for phase in fetch extract patch build run test
.  if ${_NODEJS_FEATURE_ARGS:M*${phase}*}
${phase:tu}_DEPENDS+=	${_NODEJS_FEATURE_DEP}
.  endif
. endfor

NODEJS_PKG_MANAGER=	${_NODEJS_FEATURE}
.endif # USE_NODEJS

.endif
