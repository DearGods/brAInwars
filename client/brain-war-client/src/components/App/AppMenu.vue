<template>
  <div
    class="app_menu dark_background"
    :class="{
      open: isMenuOpen,
    }"
  >
    <div class="menu_list">
      <div
        class="user_balance_wrapper light_background_trans mbm flex align_center content_space_between mpx light_text"
      >
        <span> Balance: </span>
        <span v-if="$store.getters.getUserBalance">
          ${{ $store.getters.getUserBalance }}
        </span>
      </div>
      <div class="user_name_wrapper light_background_trans mbm">
        <UserName
          :class="{
            disabled: !$store.getters.isLoggedIn,
          }"
        />
      </div>
      <div class="user_actions_wrapper">
        <div class="srm">
          <DepositButton @clicked="deposit($screen.isSmall)" />
        </div>
        <div class="slm">
          <WithdrawButton @clicked="withdraw($screen.isSmall)" />
        </div>
      </div>
      <template v-for="(item, index) in menuItems" :key="index">
        <router-link class="router" :to="item.path">
          <div class="menu_item pointer" v-if="(item.isLogged && $store.getters.isLoggedIn) || !item.isLogged" @click="$store.dispatch('toggleMenuState')">
            <span class="light_text">
              {{ item.text }}
            </span>
          </div>
        </router-link>
      </template>
    </div>
    <Icon 
      dark 
      small 
      title="sound"
      :icon="audioIcon"
      class="icon srm sound_icon" 
      @clicked="$store.dispatch('toggleGameAudio')" 
    />
    <Icon 
      dark 
      small 
      title="music"
      :icon="musicIcon"
      class="icon srm music_icon" 
      @clicked="$store.dispatch('toggleGameMusic')" 
    />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import UserName from "@/components/Game/UserName.vue";
import DepositButton from "@/components/Shared/Buttons/DepositButton.vue";
import WithdrawButton from "@/components/Shared/Buttons/WithdrawButton.vue";
import Icon from '../../components/Shared/Global/Icon.vue';
import { DepositModal } from "@/helpers/types/DepositModalType"

export default defineComponent({
  name: "AppMenu",
  components: {
    UserName,
    Icon,
    DepositButton,
    WithdrawButton
  },

  data() {
    return {
      menuItems: [
        {
          text: "HOME",
          path: "/",
        },
        {
          text: "LOGOUT",
          isLogged: true,
          path: "/logout",
        },
        // {
        //   text: 'HOW TO PLAY',
        //   path: '/how-to-play'
        // },
        // {
        //   text: 'TERMS & CONDITIONS',
        //   path: '/terms-and-conditions'
        // },
      ],
    };
  },

  computed: {
    isMenuOpen(): boolean {
      return this.$store.getters.isMenuOpen;
    },

    audioIcon(): string {
      return this.$store.getters.isAudioOn ? 'volume_up' : 'volume_off';
    },

    musicIcon(): string {
      return this.$store.getters.isMusicOn ? 'music_note' : 'music_off';
    },
  },

  methods: {
    deposit(closeMenu = false) {
      this.$store.dispatch('playAudio', 'click');
      if(closeMenu) {
        this.$store.dispatch('toggleMenuState');
      }
      this.$store.dispatch("setDepositModalMode", DepositModal.ADDING_MONEY)
    },

    withdraw(closeMenu = false) {
      this.$store.dispatch('playAudio', 'click');
      if(closeMenu) {
        this.$store.dispatch('toggleMenuState');
      }
      this.$store.dispatch("setDepositModalMode", DepositModal.WITHDRAW)
    },
  },
});
</script>

<style scoped lang="scss">
.app_menu {
  position: absolute;
  left: calc(-22vw - 5px);
  top: 0;
  height: 100vh;
  width: 22vw;
  min-width: 330px;
  transition: 0.3s left linear;
  z-index: 100;
  border-right: 2px solid #e0ded2;
  box-shadow: 4px 8px 8px 3px #000110aa;

  .sound_icon {
    position: absolute;
    left: 10px;
    bottom: 10px;
  }

  .music_icon {
    position: absolute;
    left: 40px;
    bottom: 10px;
  }

  @media only screen and (max-width: 600px) {
    left: -350px;
  }

  &.open {
    left: 0;
  }

  .menu_list {
    margin-top: 100px;

    .user_name_wrapper {
      border-radius: 4px;
      margin-left: 17px;
      width: calc(100% - 34px);
      height: 45px;
      position: relative;
      top: 7%;
    }

    .user_actions_wrapper {
      border-radius: 4px;
      margin-left: 17px;
      width: calc(100% - 34px);
      height: 45px;
      position: relative;
      display: flex;
      top: 7%;
      
      & > div {
        width: calc(50% - 11px);
      }
    }

    .user_balance_wrapper {
      border-radius: 4px;
      margin-left: 17px;
      width: calc(100% - 58px);
      height: 45px;
      position: relative;
      top: 7%;
    }

    .router {
      text-decoration: none;

      .menu_item {
        font-size: 1.2em;
        padding: 20px;
        padding-left: 30px;
        transition: 0.3s background-color;

        &:hover {
          background-color: var(--lightBlueTrans);
        }
      }
    }
  }
}
</style>
