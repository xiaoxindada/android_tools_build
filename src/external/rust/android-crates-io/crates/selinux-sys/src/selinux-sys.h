#include <selinux/selinux.h>
#include <selinux/context.h>
#include <selinux/avc.h>
#include <selinux/label.h>

#ifdef SELINUX_SYS_RESTORECON_H
#include <selinux/restorecon.h>
#endif // SELINUX_SYS_RESTORECON_H

#ifdef SELINUX_SYS_GET_CONTEXT_LIST_H
#include <selinux/get_context_list.h>
#endif // SELINUX_SYS_GET_CONTEXT_LIST_H

#ifdef SELINUX_SYS_GET_DEFAULT_TYPE_H
#include <selinux/get_default_type.h>
#endif // SELINUX_SYS_GET_DEFAULT_TYPE_H
