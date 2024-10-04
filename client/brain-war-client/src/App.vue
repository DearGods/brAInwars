<template>
  <v-app>
    <AppToolbar />
    <AppMenu />
    
    <audio loop autoplay class="background_music" @play="backgroundMusicStart()">
      <source :src="$domain.baseDomain + '/assets/audios/background_music.mp3'" type="audio/mpeg">
    </audio>
    <GameDepositModal v-if="showDepositModal" />
    <audio-player file-name="click.mp3"></audio-player>
    <router-view key="view" v-slot="{ Component }">
      <transition>
        <component
          :is="Component"
          :class="{
            mobile_app_wrapper: $screen.isSmall,
          }"
        />
      </transition>
    </router-view>

  </v-app>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { toast } from "vue3-toastify";
import "vue3-toastify/dist/index.css";
import AppToolbar from "./components/App/AppToolbar.vue";
import AppMenu from "./components/App/AppMenu.vue";
import INotification from "@/helpers/interfaces/INotification"
import AudioPlayer from './components/Shared/Global/AudioPlayer.vue';
import GameDepositModal from "@/components/Game/GameDepositModal.vue"
import { DepositModal } from "@/helpers/types/DepositModalType"

export default defineComponent({
  name: "AppPage",

  components: {
    AppToolbar,
    AppMenu,
    AudioPlayer,
    GameDepositModal,
  },

  data() {
    return {
      pendingNotifications: 0 as number,
      isNotificationActive: false as boolean,
      backgroundMusic: 0.1 as number
    };
  },

  created() {
    this.$store.dispatch("loadSettingsFromCache");
    setTimeout(() => {
      this.playBackgroundIfDidntAutoPlay();
    }, 2000);
  },

  watch: {
    notification() {
      this.showNotification(this.notification);
    },

    isMusicOn() {
      this.setMusicVolume();
    }
  },

  computed: {
    notification() {
      return this.$store.getters.getNotification;
    },

    showDepositModal(): boolean {     
      return this.$store.getters.getDepositModalMode !== DepositModal.CLOSED;
    },
    
    isMusicOn(): boolean {
      return this.$store.getters.isMusicOn;
    }
  },

  methods: {
    toggleNotificationStatus() {
      this.isNotificationActive = !this.isNotificationActive;
    },

    showNotification(notification: INotification) {
      toast(notification.content, {
        "theme": "dark",
        "type": notification.type,
        "dangerouslyHTMLString": true
      })
    },

    backgroundMusicStart() {
      this.setMusicVolume();
    },

    playBackgroundIfDidntAutoPlay() {
      const audio = document.querySelector('.background_music') as HTMLAudioElement;
      if(audio && !audio.paused) {
        this.setMusicVolume();
      }
    },

    setMusicVolume() {
      const audio = document.querySelector('.background_music') as HTMLAudioElement;
      if(audio) {
        audio.volume = this.isMusicOn ? this.backgroundMusic : 0;
        audio.play();
      }
    }
  },
});
</script>

<style scoped lang="scss">
.fade-enter-active,
.fade-leave-active {
  transition-duration: 0.3s;
  transition-property: opacity;
  transition-timing-function: ease;
}

.fade-enter,
.fade-leave-active {
  opacity: 0;
}

.mobile_app_wrapper {
  overflow-y: hidden;
  height: 100vh;
  
  @media only screen and (max-width: 600px) {
    height: calc(100vh - 80px);
  }
}
</style>
