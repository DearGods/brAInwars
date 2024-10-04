<template>
  <div class="desktop_toolbar flex align_center dark_background">
    <div class="menu_wrapper flex align_center content_space_between">
      <MenuIcon />
      <LogoutButton v-if="$store.getters.isLoggedIn" @clicked="logout()" />
      <ConnectButton v-else @clicked="connect()" />
    </div>
    <div class="logo_wrapper">
      <AppLogo @submit="$router.push('/')" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import AppLogo from "../Shared/Global/AppLogo.vue";
import MenuIcon from "../Shared/Global/MenuIcon.vue";
import ConnectButton from "../Shared/Buttons/ConnectButton.vue";
import LogoutButton from "../Shared/Buttons/LogoutButton.vue";

export default defineComponent({
  name: "DesktopToolbar",
  components: {
    AppLogo,
    MenuIcon,
    ConnectButton,
    LogoutButton,
  },

  data() {
    return {};
  },

  methods: {
    async connect() {
      this.$store.dispatch("playAudio", "click");
      await this.$store.dispatch("connectPhantom", (window as any)?.solana);
      // await this.$store.dispatch("connectPhantom", (window as any)?.backpack);
      await this.$store.dispatch("register");
      await this.$store.dispatch("login");
      await this.$store.dispatch("connectWs");
    },

    logout() {
      this.$store.dispatch("playAudio", "click");
      this.$store.dispatch("logout");
    },
  },
});
</script>

<style scoped lang="scss">
.desktop_toolbar {
  height: 70px;
  width: calc(100% - 20px);
  padding: 5px 10px;
  position: relative;
  z-index: 2;

  .menu_wrapper {
    position: relative;
    margin-left: 20px;
    margin-right: 10px;
    width: 100%;
  }

  .logo_wrapper {
    width: 70px;
    position: absolute;
    left: 0;
    right: 0;
    margin: auto;
  }

  .wallet_icon {
    margin-left: 40px;
  }
}
</style>
