#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>

#include "frida-gum.h"

void frida_gum_wrapper_test() {
  puts("gggg");
  gum_init_embedded();
  printf("..._get_type: %zu\n", gum_invocation_listener_get_type());
}

typedef struct _ArchetypalListener ArchetypalListener;

struct _ArchetypalListener {
  GObject parent;
  void (*enter)(void*,void*);
  void (*leave)(void*,void*);
  void* obj;
};

static void iface_init(gpointer g_iface, gpointer iface_data);

#define ARCHETYPAL_LISTENER_TYPE (archetypal_listener_get_type ())
G_DECLARE_FINAL_TYPE (
  ArchetypalListener,
  archetypal_listener,
  ARCHETYPAL,
  LISTENER,
  GObject)
G_DEFINE_TYPE_EXTENDED (
  ArchetypalListener,
  archetypal_listener,
  G_TYPE_OBJECT,
  0,
  G_IMPLEMENT_INTERFACE (GUM_TYPE_INVOCATION_LISTENER, iface_init))

ArchetypalListener* get_new_archetypal_listener() {
  return g_object_new(ARCHETYPAL_LISTENER_TYPE, NULL);
}

void ArchetypalListener_fill(ArchetypalListener* self,
                             void (*_enter)(void*,void*),
                             void (*_leave)(void*,void*), void* _obj) {
  self->enter = _enter;
  self->leave = _leave;
  self->obj = _obj;
}


static void archetypal_listener_class_init(ArchetypalListenerClass* cls) {
  (void) ARCHETYPAL_IS_LISTENER;
  (void) glib_autoptr_cleanup_ArchetypalListener;
}

static void archetypal_listener_init(ArchetypalListener* self) {
  //pass
}

static void on_enter(GumInvocationListener* lis, GumInvocationContext* ic) {
  ArchetypalListener* self = ARCHETYPAL_LISTENER(lis);
  self->enter(self->obj, ic);
}

static void on_leave(GumInvocationListener* lis, GumInvocationContext* ic) {
  ArchetypalListener* self = ARCHETYPAL_LISTENER(lis);
  self->leave(self->obj, ic);
}

static void iface_init(gpointer g_iface, gpointer iface_data) {
  GumInvocationListenerIface* iface = (GumInvocationListenerIface*) g_iface;
  iface->on_enter = on_enter;
  iface->on_leave = on_leave;
}


void setup_hook(GumInterceptor* interceptor,
                GumInvocationListener* lis, size_t addr) {
  gum_interceptor_begin_transaction(interceptor);
  gum_interceptor_attach_listener(
    interceptor,
    GSIZE_TO_POINTER(addr),
    lis,
    NULL
  );
  gum_interceptor_end_transaction(interceptor);
}
