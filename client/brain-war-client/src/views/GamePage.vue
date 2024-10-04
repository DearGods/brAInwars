<template>
  <div class="game_page">
    <div class="game_content">
      <video
        v-if="$store.getters.isGameLive"
        preload="auto"
        autoplay
        :src="videoSrc"
        muted
        class="game_active_loading_animation"
        loop
      >
        <span>This video is currently unavailable</span>
      </video>
      <div
        class="darkner"
        :class="{
          game_active_darkner: $store.getters.isGameLive,
        }"
      ></div>
      <img :src="imageSrc" alt="background" class="background_image" />
      <component
        v-if="$store.getters.getPlayersStatus"
        :is="$screen.isSmall ? 'GamePageMobile' : 'GamePageDesktop'"
      ></component>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "@vue/runtime-core";
import GamePageDesktop from "../components/Game/GamePageDesktop.vue";
import GamePageMobile from "@/components/Game/GamePageMobile.vue";
import { DepositModal } from "@/helpers/feTypes";

export default defineComponent({
  name: "GamePage",

  data() {
    return {
      gameCheckerInterval: undefined as NodeJS.Timer | undefined,
      lastGameStatus: null as null | string,
      gameStatuses: [] as string[],
      increasePlaybackRateInterval: undefined as NodeJS.Timer | undefined,
      videoSrc: "https://cdn.brainwars.win/game-animation.mp4",
    };
  },

  components: {
    GamePageDesktop,
    GamePageMobile,
  },

  created() {
    this.joinRoom();
    this.gameChecker();
  },

  watch: {
    isGameLive() {
      if (this.isGameLive) {
        this.gameLoadingAnimation();
      } else {
        clearInterval(this.increasePlaybackRateInterval);
      }
    },
  },

  computed: {
    imageSrc(): string {
      return "https://cdn.brainwars.win/public%2Fimages%2Fgame%2Fbackground.webp";
    },

    isGameLive(): boolean {
      return this.$store.getters.isGameLive;
    },
  },

  methods: {
    async joinRoom() {
      if (this.$store.getters.getGameId) {
        return;
      }

      await this.$store.dispatch("getGames");
      this.$store.dispatch("joinRoom", this.$route.params.id);

      if (!this.$store.getters.getGameId) {
        this.$router.push("/");
      }
    },

    gameChecker() {
      this.gameCheckerInterval = setInterval(() => {
        const game = this.$store.getters.getCurrentGame;
        const isSameStatus =
          this.lastGameStatus && this.lastGameStatus === game.game_status;
        if (isSameStatus) {
          this.$store.dispatch("getGame", this.$store.getters.getGameId);
        }

        this.lastGameStatus =
          isSameStatus && game.game_status === "WaitingForPlayers"
            ? null
            : game.game_status;
        this.gameStatuses.push(game.game_status);
      }, 5000);
    },

    gameLoadingAnimation() {
      this.increasePlaybackRateInterval = setInterval(
        this.increasePlaybackRate,
        2500,
      );
    },

    increasePlaybackRate() {
      const video = document.querySelector(
        ".game_active_loading_animation",
      ) as HTMLVideoElement;
      //if(video.playbackRate < 15) {
      video.playbackRate = video.playbackRate * 2;
      //}
      console.log({ v: video.playbackRate });
    },
  },

  beforeUnmount() {
    this.$store.dispatch("setDepositModalMode", DepositModal.CLOSED);
    clearInterval(this.gameCheckerInterval);
    clearInterval(this.increasePlaybackRateInterval);
  },
});
</script>

<style scoped lang="scss">
.game_page {
  height: 100%;
  width: 100%;
}

.game_content {
  display: flex;
  height: calc(100vh - 80px);
  width: 100vw;

  // @media only screen and (max-width: 600px) {
  //   height: calc(100vh - 80px);
  //   position: relative;
  // }

  .background_image {
    height: 100%;
    width: 100%;
    object-fit: cover;
  }

  .darkner {
    height: calc(100vh - 80px);
    width: 100vw;
    background-color: #00011088;
    position: absolute;
    left: 0;
    top: 80px;
    transition: 0.3s background-color linear;
  }

  .game_active_darkner {
    background-color: #00011055;
  }

  .game_active_loading_animation {
    position: absolute;
    width: 100vw;
    height: 100vh;
    bottom: 10vh;
    object-fit: cover;
  }
}
</style>
