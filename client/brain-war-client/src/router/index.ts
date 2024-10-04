import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "home",
    component: () => import("../views/HomePage.vue"),
  },
  {
    path: "/about",
    name: "about",
    component: () => import("../views/AboutPage.vue"),
  },
  {
    path: "/room/:id",
    name: "game",
    component: () => import("../views/GamePage.vue"),
  },
  {
    path: "/test",
    name: "test",
    component: () => import("../views/TestPage.vue"),
  },
  {
    path: "/logout",
    name: "logout",
    component: () => import("../views/LogoutPage.vue"),
  },
  {
    path: "/:pathMatch(.*)*",
    name: "Home",
    component: () => import("../views/HomePage.vue"),
    beforeEnter: (to, from, next) => {
      // Check if the path starts with 'api', if so, continue to server (ignore in Vue Router)
      if (to.path.startsWith("/api")) {
        // Here, you might want to redirect or just do nothing
        // For example, next(false) would effectively do nothing
        next(false);
      } else {
        // For other paths, continue with the routing
        next();
      }
    },
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
