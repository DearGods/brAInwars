<template>
  <div class="game_timer flex align_center content_center">
    <h2 ref="timer" class="game_timer_text white_text">
      {{ formattedTime }}
    </h2>
    <h2 class="white_text" v-if="!formattedTime">
      {{ subtext }}
    </h2>
  </div>
  <audio class="counter_music">
    <source
      src="https://cdn.brainwars.win/public%2Fassets%2Faudios%2Fcounter.mp3"
      type="audio/mpeg"
    />
  </audio>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "GameCounter",

  emits: ["gameAboutToStart"],

  data() {
    return {
      timeInMilliseconds: 20000, // 20 seconds
      countdownInterval: undefined as NodeJS.Timer | undefined,
      formattedTime: "" as string,
      subtext: "",
      startTime: null as null | number,
      setGameAboutToStart: false as boolean,
      isCounterMusicOn: false as boolean,
      counterMusic: 0.4 as number,
    };
  },

  created() {
    this.startTime = Date.now();
    this.updateCounter();
  },

  watch: {
    getStartTime() {
      this.updateCounter();
    },

    getGameCounter(value) {
      this.timeInMilliseconds = value;
    },

    isMusicOn() {
      this.setMusicVolume();
    },

    isCounterMusicOn() {
      this.setMusicVolume();
    },
  },

  computed: {
    getStartTime() {
      return this.$store.getters.getGameStartTime;
    },

    getGameCounter() {
      return this.$store.getters.getGameCounter;
    },

    isMusicOn(): boolean {
      return this.$store.getters.isMusicOn;
    },
  },

  methods: {
    updateCounter() {
      if (!this.$store.getters.getGameStartTime) {
        return;
      }

      const endTime = Date.now();
      const startTime = new Date(this.$store.getters.getGameStartTime);
      const diff =
        startTime.getTime() + this.$store.getters.getGameCounter - endTime;

      if (diff >= 0) {
        if (diff <= 10000 && !this.isCounterMusicOn) {
          this.isCounterMusicOn = true;
        }

        if (!this.setGameAboutToStart && diff < 3000) {
          // 3 seconds
          this.setGameAboutToStart = true;

          this.$emit("gameAboutToStart");
        }

        const seconds = Math.floor(diff / 1000);
        const milliseconds = Math.floor((diff % 1000) / 10);

        this.formattedTime = `${seconds.toString().padStart(2, "0")}:${milliseconds.toString().padStart(2, "0")}`;
        setTimeout(this.updateCounter, 10); // Update roughly every millisecond
      } else {
        if (!this.setGameAboutToStart) {
          return;
        }
        this.isCounterMusicOn = false;
        setTimeout(() => {
          this.formattedTime = "";
          this.subtext = "STARTING THE GAME";
        }, 300);
      }
    },

    setMusicVolume() {
      if (this.isCounterMusicOn) {
        this.play();
      } else {
        this.stop();
      }
    },

    play() {
      const audio = document.querySelector(
        ".counter_music",
      ) as HTMLAudioElement;
      if (!audio) {
        return;
      }

      audio.volume = this.isMusicOn ? this.counterMusic : 0;
      if (audio.volume) {
        audio.play();
      }
    },

    stop() {
      const audio = document.querySelector(
        ".counter_music",
      ) as HTMLAudioElement;
      if (audio) {
        audio.pause();
      }
    },
  },
});
</script>

<style scoped lang="scss">
.game_timer {
  position: relative;
  width: 100%;
  height: 100%;

  .game_timer_text {
    font-size: 5em;
  }
}
</style>
