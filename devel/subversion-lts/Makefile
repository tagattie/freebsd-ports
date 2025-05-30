PORTREVISION=	1
PKGNAMESUFFIX=	-lts

MAINTAINER=	michaelo@apache.org
COMMENT=	Version control system, long-term-support version
WWW=		https://subversion.apache.org/

CONFLICTS_INSTALL=	subversion

USE_RC_SUBR=	svnserve

PORTDOCS=	BUGS CHANGES COMMITTERS INSTALL README

OPTIONS_DEFINE=	\
		BDB		\
		DOCS		\
		GPG_AGENT 	\
		NLS		\
		SASL		\
		SERF		\
		SVNSERVE_WRAPPER\
		TEST		\
		TOOLS

OPTIONS_DEFAULT=GPG_AGENT SERF TOOLS

GPG_AGENT_DESC=		Enable GPG agent password store
SERF_DESC=		WebDAV/Delta-V (HTTP/HTTPS) repo access module
SVNSERVE_WRAPPER_DESC=	Enable svnserve wrapper (umask setter)
TEST_DESC=		Run subversion test suite
TOOLS_DESC=		Install several tools

SVNSERVE_WRAPPER_EXTRA_PATCHES=	${PATCHDIR}/extra-patch-svnserve-wrapper

USES+=			shebangfix python:env
python_OLD_CMD+=	"/bin/env python"
SHEBANG_REGEX=		./tools/.*

.include "Makefile.common"

SVNREPOS?=	/home/svn/repos
SVNFSTYPE?=	fsfs
SVNGROUP?=	svn
SVNUSER?=	svn

GNU_CONFIGURE_MANPREFIX=	${PREFIX}/share
CONFIGURE_ARGS+=	--without-swig-perl \
			--without-swig-python \
			--without-swig-ruby \
			--without-gnome-keyring \
			--without-kwallet \
			--with-apxs=no

.if ${PORT_OPTIONS:MSVNSERVE_WRAPPER}
SUB_LIST+=	SVNSERVE_BIN_EXT=".bin"
PLIST_SUB+=	NO_SVNSERVE_WRAPPER="@comment "
SVNSERVE_EXT=	.bin
.else
SUB_LIST+=	SVNSERVE_BIN_EXT=""
PLIST_SUB+=	NO_SVNSERVE_WRAPPER=""
SVNSERVE_EXT=
.endif

.if ${PORT_OPTIONS:MTEST}
TEST_DEPENDS=	${PYTHON_PKGNAMEPREFIX}sqlite3>0:databases/py-sqlite3@${PY_FLAVOR}
.endif

.if make(repository)
WITH_REPOSITORY_CREATION=	yes
.endif

.if defined(WITH_REPOSITORY_CREATION)
MKREPOS_TARGET=	_mkrepos
.endif

BINS_TO_STRIP= svn svnadmin svnbench svndumpfilter svnfsfs svnlook svnmucc \
		svnrdump svnserve${SVNSERVE_EXT} svnsync svnversion

TOOLS_TO_STRIP=	fsfs-access-map svn-mergeinfo-normalizer \
		 svn-populate-node-origins-index svnauthz \
		 svnauthz-validate svnconflict svndiff svndiff3 \
		 svndiff4 svnmover svnraisetreeconflict x509-parser

pre-everything::
	@${ECHO_MSG} ""
.if defined(WITH_REPOSITORY_CREATION)
	@${ECHO_MSG} "I will create (or use, if it exists) '${SVNUSER}' user and '${SVNGROUP}' group."
	@${ECHO_MSG} "Make sure that all committers are its members."
	@${ECHO_MSG} ""
	@${ECHO_MSG} "Repository will be created at '${SVNREPOS}' with user '${SVNUSER}' and group '${SVNGROUP}'."
	@${ECHO_MSG} "Type of repository will be '${SVNFSTYPE}'."
	@${ECHO_MSG} "You could change these settings by defining SVNREPOS, SVNGROUP and SVNFSTYPE."
.else
	@${ECHO_MSG} "You can have the repository created for you by defining"
	@${ECHO_MSG} "WITH_REPOSITORY_CREATION."
	@${ECHO_MSG} ""
	@${ECHO_MSG} "Make sure that:"
	@${ECHO_MSG} "* all your svn users are members of a common group"
	@${ECHO_MSG} "* this group is the group id of the db/ and locks/"
	@${ECHO_MSG} "  subdirectories of your repository"
	@${ECHO_MSG} "* the above subdirectories are writable by this group"
.endif

	@${ECHO_MSG} ""
	@${ECHO_MSG} "Many useful scripts will be installed into ${DATADIR}"

pre-configure:
.if ${PORT_OPTIONS:MBDB}
	@BDB_VERSION=`${APR_APU_DIR}/${APU_CONFIG} --db-version`; \
	if [ "$${BDB_VERSION}" != "5" -a "$${BDB_VERSION}" != "185" ] ; then \
		${ECHO_MSG} "" ; \
		${ECHO_MSG} 'You should build `'"devel/apr1' with Berkeley DB (5, or 18) support to use subversion with it." ; \
		${ECHO_MSG} 'Please rebuild `'"devel/apr1' with option "'`'"BDB' and try again." ; \
		${ECHO_MSG} "" ; \
		${ECHO_MSG} "Or you can disable Berkeley DB support. Only 'fs' repository backend will be available." ; \
		${ECHO_MSG} "" ; \
		${FALSE} ; \
	fi
.endif

post-configure:
# Enable GNOME Keyring / KWallet support in main code, as it is harmless
	@${REINPLACE_CMD} -E 's%.*#undef SVN_HAVE_(GNOME_KEYRING|KWALLET).*%#define SVN_HAVE_\1 1%' ${WRKSRC}/subversion/svn_private_config.h
# ugly hack: save tools directory so we don't have to cleanup before installing
	@${CP} -R ${WRKSRC}/tools ${WRKSRC}/tools.examples
	@${CHMOD} -R a-st,o+rX ${WRKSRC}/tools.examples

# takes a long time, and will fail but gives maintainer a good oversight
.if ${PORT_OPTIONS:MTEST}
test: build
	-@make check -C ${WRKSRC}
.endif

post-build-TOOLS-on:
	@${MAKE} -C ${WRKSRC} tools ${MAKE_ARGS}

post-install:	${MKREPOS_TARGET}
	for F in ${BINS_TO_STRIP} ; do \
	 ${STRIP_CMD} "${STAGEDIR}${PREFIX}/bin/$${F}" ; \
	done
	@${STRIP_CMD} ${STAGEDIR}${PREFIX}/lib/*.so
	@${MKDIR} ${STAGEDIR}${DATADIR}
	(cd ${WRKSRC}/tools.examples && ${TAR} --exclude '*.in' -cf - * | ${TAR} -C ${STAGEDIR}${DATADIR} -xof - )
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_DATA} ${PORTDOCS:S|^|${WRKSRC}/|} ${STAGEDIR}${DOCSDIR}

post-install-SVNSERVE_WRAPPER-on:
	@${INSTALL_SCRIPT} ${FILESDIR}/svnserve.wrapper ${STAGEDIR}${PREFIX}/bin/svnserve.sample

post-install-TOOLS-on:
	@${MAKE} -C ${WRKSRC} install-tools ${MAKE_ARGS}
	for F in ${TOOLS_TO_STRIP} ; do \
	 ${STRIP_CMD} "${STAGEDIR}${PREFIX}/bin/$${F}" ; \
	done

# ===============================================================================
repository:	_mkrepos

_SVNGRPFILES=	db locks locks/db.lock locks/db-logs.lock
SVNGRPFILES=	${_SVNGRPFILES:S,^,${SVNREPOS}/,}

_mkrepos:	.USE
.if !exists(${SVNREPOS})
.if ${PORT_OPTIONS:MSVNSERVE_WRAPPER}
	@if /usr/sbin/pw groupshow "${SVNGROUP}" >/dev/null 2>&1; then \
	    ${ECHO_MSG} "You already have a group \"${SVNGROUP}\", so I will use it."; \
	else \
	    if /usr/sbin/pw groupadd ${SVNGROUP} -h -; \
	    then \
	        ${ECHO_MSG} "Added group \"${SVNGROUP}\"."; \
	    else \
	        ${ECHO_MSG} "Adding group \"${SVNGROUP}\" failed..."; \
	        ${ECHO_MSG} "Please create it, and try again."; \
	        ${FALSE}; \
	    fi; \
	fi
	@if /usr/sbin/pw usershow "${SVNUSER}" >/dev/null 2>&1; then \
	    ${ECHO_MSG} "You already have a user \"${SVNUSER}\", so I will use it."; \
	else \
	    if /usr/sbin/pw useradd ${SVNUSER} -h -; \
	    then \
	        ${ECHO_MSG} "Added user \"${SVNUSER}\"."; \
	    else \
	        ${ECHO_MSG} "Adding user \"${SVNUSER}\" failed..."; \
	        ${ECHO_MSG} "Please create it, and try again."; \
	        ${FALSE}; \
	    fi; \
	fi
.endif
	@${MKDIR} ${SVNREPOS}
	@${PREFIX}/bin/svnadmin create --fs-type ${SVNFSTYPE} ${SVNREPOS}
.if ${PORT_OPTIONS:MSVNSERVE_WRAPPER}
	@${CHOWN} ${SVNUSER}:${SVNGROUP} ${SVNGRPFILES}
	@${CHMOD} g+w ${SVNGRPFILES}
	@for i in ${SVNREPOS}/db/* ; do \
	    i=$${i##*/}; \
	    case $$i in \
	    DB_CONFIG|fs-type|uuid) ;; \
	    *) ${CHOWN} -R ${SVNUSER}:${SVNGROUP} ${SVNREPOS}/db/$$i; \
	       ${CHMOD} -R g+w ${SVNREPOS}/db/$$i; \
	       ;; \
	    esac; \
	done
.endif
.endif

svn-build-outputs-hack:
	(cd ${WRKSRC} && python gen-make.py --release --installed-libs libsvn_client,libsvn_delta,libsvn_diff,libsvn_fs,libsvn_fs_x,libsvn_ra,libsvn_repos,libsvn_subr,libsvn_wc)
	${MV} ${WRKSRC}/build-outputs.mk ${FILESDIR}/build-outputs.mk.addons

.include <bsd.port.post.mk>
