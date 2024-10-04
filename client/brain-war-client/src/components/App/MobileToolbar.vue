<template>
  <div class="mobile_toolbar flex align_center dark_background">
    <div class="menu_wrapper flex align_center content_space_between">
      <MenuIcon />
      <div class="flex align_center">
        <template v-if="$route.query.tab">
          <Icon
            dark
            icon="message"
            class="icon srm message_icon"
            @clicked="toggleChat()"
          />
        </template>
        <template v-if="!$store.getters.isLoggedIn">
          <ConnectButton v-if="$screen.isMediumAndUp" @clicked="connect()" />
          <Icon
            dark
            v-else
            icon="wallet"
            class="icon wallet_icon"
            @clicked="connect()"
          />
        </template>
      </div>
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
import Icon from "../../components/Shared/Global/Icon.vue";

export default defineComponent({
  name: "MobileToolbar",
  components: {
    AppLogo,
    MenuIcon,
    ConnectButton,
    Icon,
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

    toggleChat() {
      this.$store.dispatch("playAudio", "click");
      this.$store.dispatch("toggleChat");
    },
  },
});
</script>

<style scoped lang="scss">
.mobile_toolbar {
  height: 70px;
  width: calc(100% - 20px);
  padding: 5px 10px;
  position: relative;

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
    top: -8px;
  }

  .message_icon {
    top: -5px;
  }
}
</style>
