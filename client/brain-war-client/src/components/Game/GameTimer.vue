<template>
  <div class="game_timer flex align_center content_center">
    <h2 ref="timer" class="game_timer_text white_text"></h2>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "GameTimer",

  data() {
    return {
      timeInMilliseconds: 0,
      countdownInterval: undefined as NodeJS.Timer | undefined,
    };
  },

  mounted() {
    this.start();
  },

  methods: {
    start() {
      if(this.$store.getters.getGameStartTime) {
        this.timeInMilliseconds = Math.floor((Date.now() - this.$store.getters.getGameStartTime.getTime()));
        this.countdownInterval = setInterval(this.updateCountdown, 10);
      }
    },

    stop() {
      clearInterval(this.countdownInterval);
    },

    updateCountdown() {
      if (!this.$refs.timer) {
        return;
      }

      const seconds = Math.floor(this.timeInMilliseconds / 1000);
      const milliseconds = Math.floor((this.timeInMilliseconds % 1000) / 10);
      const formattedTime = `${seconds.toString().padStart(2, "0")}:${milliseconds.toString().padStart(2, "0")}`;

      (this.$refs.timer as HTMLDivElement).innerHTML = `${formattedTime}`;
      this.timeInMilliseconds += 10;
    },
  },
});
</script>

<style scoped lang="scss">
.game_timer {
  position: relative;
  width: 100%;
  height: 100%;
}
</style>
