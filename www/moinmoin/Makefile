PORTNAME=	moinmoin
PORTVERSION=	1.9.11
PORTREVISION=	1
CATEGORIES=	www python
MASTER_SITES=	https://static.moinmo.in/files/
DISTNAME=	moin-${PORTVERSION}

MAINTAINER=	python@FreeBSD.org
COMMENT=	Easy to use, full-featured and extensible wiki software package
WWW=		https://moinmo.in/

LICENSE=	GPLv2
LICENSE_FILE=	${WRKSRC}/docs/licenses/COPYING

USES=		cpe python:2.7 shebangfix
USE_PYTHON=	distutils

NO_ARCH=	yes
CPE_VENDOR=	moinmo
PYDISTUTILS_PKGNAME=	moin
SHEBANG_FILES=	wiki/server/moin*

PLIST_SUB+=	PYTHON_SITELIB=${PYTHON_SITELIBDIR:S|^${LOCALBASE}/||} \
		DATADIR=share/moin
SUB_FILES=	pkg-install
SUB_LIST=	MOINDIR=${MOINDIR} MOINDEST=${MOINDEST} MOINVER=${MOINVER} \
		HTDOCS=${PYTHON_SITELIBDIR}/MoinMoin/web/static/htdocs

OPTIONS_DEFINE=		RECAPTCHA VERIFYEMAIL
OPTIONS_DEFAULT=	RECAPTCHA VERIFYEMAIL
OPTIONS_SUB=		yes

RECAPTCHA_DESC=		reCAPTCHA v2 support
VERIFYEMAIL_DESC=	Support for verifying new account email addresses

RECAPTCHA_EXTRA_PATCHES=	${PATCHDIR}/extra-patch-recaptchav2
VERIFYEMAIL_EXTRA_PATCHES=	${PATCHDIR}/extra-patch-verifyemail

CGIUSER?=	www
CGIGROUP?=	www
MOINDIR=	${PREFIX}/share/moin
MOINDEST?=	${PREFIX}/www/wiki
MOINVER?=	${PORTVERSION:S|.||g}
MOINTYPE?=	CGI

PKGDEINSTALL=	${PKGINSTALL}

.if ${MOINTYPE} == "STANDALONE"
MOINSCRIPT=	${MOINDIR}/server/moin
.elif ${MOINTYPE} == "FCGI"
MOINSCRIPT=	${MOINDIR}/server/moin.fcgi
.elif ${MOINTYPE} == "MOD_PYTHON"
.elif ${MOINTYPE} == "CGI"
MOINSCRIPT=	${MOINDIR}/server/moin.cgi
.elif ${MOINTYPE} == "WSGI"
MOINSCRIPT=	${MOINDIR}/server/moin.wsgi
.else
BROKEN=		MOINTYPE must be a STANDALONE, FCGI, WSGI, MOD_PYTHON, or CGI
.endif

pre-everything::
	@${ECHO}
	@${ECHO} "Set MOINTYPE=(CGI|FCGI|WSGI|STANDALONE) to define"
	@${ECHO} "type of installation. Default is CGI."
	@${ECHO} "Use MOINDEST=/path to modify installation destination."
	@${ECHO} "Default value for MOINDEST is ${PREFIX}/www/wiki."
	@${ECHO}
	@${ECHO} "To get correct permissions, please set CGIUSER, CGIGROUP"
	@${ECHO} "per default it is set to www:www."
	@${ECHO}

instance: pre-everything apply-slist
	@${ECHO_CMD} "Creating a new wiki instance in ${MOINDEST}."; \
	if [ -f ${MOINDIR}/config/wikiconfig.py ]; then \
		${MKDIR} ${MOINDEST}; \
		${CP} -R ${MOINDIR}/data ${MOINDEST}; \
		${CP} -R ${MOINDIR}/underlay ${MOINDEST}; \
		${CHMOD} -R u+rw,go-ws ${MOINDEST}/data; \
		${INSTALL_SCRIPT} ${MOINDIR}/config/wikiconfig.py ${MOINDEST}; \
		if [ ! -z ${MOINSCRIPT} ]; then \
			${INSTALL_SCRIPT} ${MOINSCRIPT} ${MOINDEST}; \
		fi; \
		${CHOWN} -R ${CGIUSER}:${CGIGROUP} ${MOINDEST}; \
		${SH} ${PKGINSTALL} ${PKGNAME} INSTANCE ${MOINTYPE}; \
	else \
		${ECHO_CMD} "You need to install moinmoin first before trying"; \
		${ECHO_CMD} "to add a new wiki instance."; \
	fi

.include <bsd.port.mk>
