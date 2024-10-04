<template>
  <div class="toolbar">
    <component :is="toolbar" />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import Screen from "@/helpers/Screen";
import MobileToolbar from "./MobileToolbar.vue";
import DesktopToolbar from "./DesktopToolbar.vue";
export default defineComponent({
  name: "AppToolbar",
  components: {
    MobileToolbar,
    DesktopToolbar,
  },

  data() {
    return {
      components: {
        mobile: "MobileToolbar",
        desktop: "DesktopToolbar",
      } as Record<string, string>,
    };
  },

  computed: {
    toolbar(): string {
      return Screen.isMobile()
        ? this.components.mobile
        : this.components.desktop;
    },
  },

  methods: {},
});
</script>

<style scoped lang="scss">

  .toolbar {
    z-index: 101;
    position: relative;
  }

</style>
