<template>
  <template v-if="$store.getters.isGameFinished && winnerName">
    <div class="fireworks_wrapper">
      <div
        class="fireworks_content"
        :class="{
          fireworks_content_mobile: $screen.isSmall,
        }"
      >
        <div class="firework"></div>
        <div class="firework"></div>
        <div class="firework"></div>
      </div>
    </div>
    <div class="winner_wrapper white_text">
      <div class="text_center">
        <span> The Winner is </span>
        <br />
        <h2 class="winner_name ellipsis">
          {{ winnerName }}
        </h2>
        <div class="prize_info" v-if="showPrize">
          <span> With a prize of: </span>
          <br />
          <h3>
            {{ winnerPrize }}
          </h3>
        </div>
      </div>
    </div>
  </template>
  <div
    v-if="$store.getters.isGameLive"
    class="game_display"
    :class="{
      game_display_active: isTimerRunning,
      game_display_active_mobile: $screen.isSmall,
    }"
  >
    <!-- <audio loop autoplay class="rocket_audio" @play="rocketAudioStart()">
      <source
        src="https://cdn.brainwars.win/public%2Fassets%2Faudios%2Frocket.mp3"
        type="audio/mpeg"
      />
    </audio> -->
    <GameTimer ref="timer" class="timer" />
    <!-- <div class="rocket" v-if="isTimerRunning">
      <div class="rocket-extras"></div>
      <div class="jet"><span></span></div>
    </div> -->
  </div>
  <div
    v-if="$store.getters.isGamePending"
    class="game_countdown game_countdown_box dark_background_trans rounded"
  >
    <GameCountdown
      v-if="$store.getters.getGameStartTime && !$store.getters.isGameFinished"
      @gameAboutToStart="gameAboutToStart()"
    />
    <h2
      class="waiting_for_players white_text text_center"
      v-if="!$store.getters.getGameStartTime"
    >
      WAITING FOR PLAYERS
    </h2>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import GameTimer from "./GameTimer.vue";
import GameCountdown from "@/components/Game/GameCountdown.vue";

export default defineComponent({
  name: "GameDisplay",
  components: {
    GameTimer,
    GameCountdown,
  },

  data() {
    return {
      rocketVolume: 0.2,
    };
  },

  watch: {
    isTimerRunning() {
      if (this.isTimerRunning) {
        return;
      }

      if (!this.$refs.timer) {
        return;
      }

      (this.$refs.timer as typeof GameTimer).stop();
    },

    // isAudioOn() {
    //   this.setAudioVolume();
    // },
  },

  computed: {
    isTimerRunning() {
      return this.$store.getters.isTimerRunning;
    },

    winnerName(): string {
      return this.$store.getters.getWinnerName
        ? this.$store.getters.getWinnerName.substring(0, 30)
        : "";
    },

    showPrize() {
      return this.winnerName.toLocaleLowerCase() !== "the house";
    },

    winnerPrize(): string {
      return this.$store.getters.getWinnerPrize;
    },

    // isAudioOn(): boolean {
    //   return this.$store.getters.isAudioOn;
    // },
  },
  methods: {
    // rocketAudioStart() {
    //   this.setAudioVolume();
    // },

    // setAudioVolume() {
    //   const audio = document.querySelector(".rocket_audio") as HTMLAudioElement;

    //   if (audio) {
    //     audio.volume = this.isAudioOn ? this.rocketVolume : 0;
    //   }
    // },

    gameAboutToStart() {
      this.$store.dispatch("gameAboutToStart");
    },
  },
});
</script>

<style scoped lang="scss">
@import url("https://fonts.googleapis.com/css?family=Ubuntu:400,400i,700,700i");

.game_countdown_box {
  position: fixed;
  top: 25vh;
  left: 35vw;
  width: 30vw;
  height: 10vw;
  border: 2px solid #e0ded2;
  transition: 0.3s all linear;
  display: flex;
  justify-content: center;
  align-items: center;

  @media only screen and (max-width: 600px) {
    height: 16%;
    width: 90%;
    top: calc(12% + 5px);
    left: 5%;
  }
}

.game_countdown_box_active {
  width: 50vw;
  left: 25vw;
  height: 25vh;
}

.waiting_for_players {
  font-size: 2em;
}

.timer {
  position: relative;
  top: -70px;
  font-size: 2em;
}

* {
  box-sizing: border-box;
}

.game_display_active {
  animation: moveParticles 6s linear infinite;
  border: 1px solid rgba(0, 0, 0, 0.1);
  background:
    linear-gradient(90deg, gray, transparent 10%) 0 20%/180% 0.2rem repeat-x,
    linear-gradient(90deg, gray, transparent 20%) 0 80%/150% 0.2rem repeat-x,
    linear-gradient(90deg, gray, transparent 5%) 0 65%/100% 0.2rem repeat-x,
    linear-gradient(90deg, gray, transparent 5%) 0 40%/220% 0.2rem repeat-x,
    linear-gradient(0, transparent, transparent);
  box-shadow: inset 0 0 60px 0 rgba(0, 0, 0, 0.1);
}

.game_display {
  // Moving particles
  border-radius: 50%;
  height: 70vh;
  left: 50%;
  position: absolute;
  text-align: center;
  top: 40%;
  transform: translate(-50%, -50%);
  width: 100%;
  overflow: hidden;
}

.game_display_active_mobile {
  top: 15%;
}

.winner_wrapper {
  position: absolute;
  width: 30%;
  height: 30%;
  background-image: url("https://cdn.brainwars.win/public%2Fimages%2Fgame%2Fwinner_frame.webp");
  background-size: 100% 100%;
  left: 35%;
  top: 15%;
  display: flex;
  align-items: center;
  justify-content: center;

  & > div {
    width: 80%;
  }

  .winner_name {
    width: 100%;
    font-size: 2em;

    @media only screen and (max-width: 600px) {
      font-size: 1.3em;
    }
  }

  .prize_info {
    position: relative;
    top: 20px;

    @media only screen and (max-width: 600px) {
      top: 10px;
    }

    h3 {
      font-size: 1.5em;
      color: var(--golden);
      position: relative;
      top: 10px;

      @media only screen and (max-width: 600px) {
        font-size: 1.2em;
        top: 0;
      }
    }
  }

  @media only screen and (max-width: 600px) {
    width: 90%;
    left: 5%;
    top: 5px;
    height: 20%;
  }
}

.fireworks_wrapper {
  position: absolute;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  top: 0;
}

.fireworks_content {
  color: #fff;
  position: relative;
  text-align: center;
  left: 0;
  top: 30vh;
  margin: auto;
}

.fireworks_content_mobile {
  top: 5%;
}

@keyframes moveParticles {
  100% {
    background-position-x: -500rem;
  }
}

.rocket {
  animation: moveRocket 2s linear infinite;
  background: lightgray;
  background: linear-gradient(
    darken(#973ac2, 20),
    #a055bb,
    darken(#b966d7, 20)
  );
  border-left: 3px solid rgba(0, 0, 0, 0.4);
  border-radius: 50%/30%;
  height: 7%;
  left: 50%;
  position: absolute;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 35%;

  &::before,
  &::after {
    content: "";
    position: absolute;
  }

  &::before {
    // Rocket fin
    animation: rotateFins 1s infinite;
    background: darken(lightgray, 10);
    background: linear-gradient(
      darken(#973ac2, 20),
      #a055bb,
      darken(#b966d7, 20)
    );
    border: 2px solid transparent;
    border-radius: 0 50% 50% 0;
    height: 140%;
    top: 50%;
    transform: translate(0, -50%);
    left: 6px;
    width: 20%;
  }

  &::after {
    // Rocket tip
    border: 7px solid transparent;
    border-left: 14px solid rgba(0, 0, 0, 0.4);
    border-radius: 15%;
    right: -16px;
    top: 2px;
  }

  &-extras {
    // Rocket body mark
    animation: moveExtras 1s infinite;
    background: rgba(0, 0, 0, 0.4);
    height: 2px;
    left: 12px;
    margin: -2px 0 0;
    position: absolute;
    top: 50%;
    transform: translate(0, -50%);
    width: 10px;

    &::before,
    &::after {
      content: "";
      position: absolute;
    }

    &::before {
      // Rocket eye
      background: white;
      border-radius: 50%;
      height: 5px;
      right: -7px;
      top: -1px;
      width: 5px;
    }

    &::after {
      // Rocket 3rd small fin
      background: darken(#29b79e, 10);
      border-top: 1px solid darken(#29b79e, 30);
      height: 1px;
      left: -10px;
      top: 1px;
      width: 6px;
    }
  }
}

@keyframes moveRocket {
  0%,
  100% {
    transform: translate(-50%, calc(-50% - 1rem));
  }

  50% {
    transform: translate(-50%, calc(-50% + 1rem));
  }
}

@keyframes rotateFins {
  0%,
  100% {
    height: 140%;
  }

  50% {
    border-top: 2px solid darken(#29b79e, 30);
    border-bottom: 2px solid darken(#29b79e, 30);
    height: 110%;
  }
}

@keyframes moveExtras {
  0%,
  100% {
    transform: translate(0, calc(-50% + 0.1rem));
  }

  50% {
    transform: translate(0, calc(-50% - 0.1rem));
  }
}

.jet {
  height: 10px;
  left: -10px;
  position: absolute;
  top: calc(50% - 5px);
  width: 10px;

  &::before,
  &::after,
  span {
    animation: moveSmoke 0.3s infinite;
    background: darken(rgb(243, 79, 4), 6);
    border-radius: 50%;
    content: "";
    filter: blur(2px);
    height: 8px;
    left: -6px;
    opacity: 1;
    position: absolute;
    transform: translate(0, 0) scale(1);
    top: 1px;
    width: 15px;
  }

  &::after {
    animation-delay: 0.1s;
  }

  span {
    animation-delay: 0.2s;
  }
}

@keyframes moveSmoke {
  100% {
    filter: blur(3px);
    opacity: 0;
    transform: translate(-40px, 0) scale(2);
  }
}

@keyframes blink {
  0%,
  100% {
    opacity: 1;
  }

  50% {
    opacity: 0.2;
  }
}

@keyframes firework {
  0% {
    transform: translate(var(--x), var(--initialY));
    width: var(--initialSize);
    opacity: 1;
  }

  50% {
    width: 0.5vmin;
    opacity: 1;
  }

  100% {
    width: var(--finalSize);
    opacity: 0;
  }
}

/* @keyframes fireworkPseudo {
  0% { transform: translate(-50%, -50%); width: var(--initialSize); opacity: 1; }
  50% { width: 0.5vmin; opacity: 1; }
  100% { width: var(--finalSize); opacity: 0; }
}
 */
.firework,
.firework::before,
.firework::after {
  --initialSize: 0.5vmin;
  --finalSize: 45vmin;
  --particleSize: 0.2vmin;
  --color1: yellow;
  --color2: khaki;
  --color3: white;
  --color4: lime;
  --color5: gold;
  --color6: mediumseagreen;
  --y: -30vmin;
  --x: -50%;
  --initialY: 60vmin;
  content: "";
  animation: firework 2s 3 ease-in-out;
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, var(--y));
  width: var(--initialSize);
  aspect-ratio: 1;
  background: /*
    radial-gradient(circle, var(--color1) var(--particleSize), #0000 0) 0% 0%,
    radial-gradient(circle, var(--color1) var(--particleSize), #0000 0) 100% 0%,
    radial-gradient(circle, var(--color1) var(--particleSize), #0000 0) 100% 100%,
    radial-gradient(circle, var(--color1) var(--particleSize), #0000 0) 0% 100%,
    */
    radial-gradient(circle, var(--color1) var(--particleSize), #0000 0) 50% 0%,
    radial-gradient(circle, var(--color2) var(--particleSize), #0000 0) 100% 50%,
    radial-gradient(circle, var(--color3) var(--particleSize), #0000 0) 50% 100%,
    radial-gradient(circle, var(--color4) var(--particleSize), #0000 0) 0% 50%,
    /* bottom right */
      radial-gradient(circle, var(--color5) var(--particleSize), #0000 0) 80%
      90%,
    radial-gradient(circle, var(--color6) var(--particleSize), #0000 0) 95% 90%,
    radial-gradient(circle, var(--color1) var(--particleSize), #0000 0) 90% 70%,
    radial-gradient(circle, var(--color2) var(--particleSize), #0000 0) 100% 60%,
    radial-gradient(circle, var(--color3) var(--particleSize), #0000 0) 55% 80%,
    radial-gradient(circle, var(--color4) var(--particleSize), #0000 0) 70% 77%,
    /* bottom left */
      radial-gradient(circle, var(--color5) var(--particleSize), #0000 0) 22%
      90%,
    radial-gradient(circle, var(--color6) var(--particleSize), #0000 0) 45% 90%,
    radial-gradient(circle, var(--color1) var(--particleSize), #0000 0) 33% 70%,
    radial-gradient(circle, var(--color2) var(--particleSize), #0000 0) 10% 60%,
    radial-gradient(circle, var(--color3) var(--particleSize), #0000 0) 31% 80%,
    radial-gradient(circle, var(--color4) var(--particleSize), #0000 0) 28% 77%,
    radial-gradient(circle, var(--color5) var(--particleSize), #0000 0) 13% 72%,
    /* top left */
      radial-gradient(circle, var(--color6) var(--particleSize), #0000 0) 80%
      10%,
    radial-gradient(circle, var(--color1) var(--particleSize), #0000 0) 95% 14%,
    radial-gradient(circle, var(--color2) var(--particleSize), #0000 0) 90% 23%,
    radial-gradient(circle, var(--color3) var(--particleSize), #0000 0) 100% 43%,
    radial-gradient(circle, var(--color4) var(--particleSize), #0000 0) 85% 27%,
    radial-gradient(circle, var(--color5) var(--particleSize), #0000 0) 77% 37%,
    radial-gradient(circle, var(--color6) var(--particleSize), #0000 0) 60% 7%,
    /* top right */
      radial-gradient(circle, var(--color1) var(--particleSize), #0000 0) 22%
      14%,
    radial-gradient(circle, var(--color1) var(--particleSize), #0000 0) 45% 20%,
    radial-gradient(circle, var(--color1) var(--particleSize), #0000 0) 33% 34%,
    radial-gradient(circle, var(--color1) var(--particleSize), #0000 0) 10% 29%,
    radial-gradient(circle, var(--color1) var(--particleSize), #0000 0) 31% 37%,
    radial-gradient(circle, var(--color1) var(--particleSize), #0000 0) 28% 7%,
    radial-gradient(circle, var(--color1) var(--particleSize), #0000 0) 13% 42%;
  background-size: var(--initialSize) var(--initialSize);
  background-repeat: no-repeat;
}

.firework::before {
  --x: -50%;
  --y: -50%;
  --initialY: -50%;
  /*   transform: translate(-20vmin, -2vmin) rotate(40deg) scale(1.3) rotateY(40deg); */
  transform: translate(-50%, -50%) rotate(40deg) scale(1.3) rotateY(40deg);
  /*   animation: fireworkPseudo 2s infinite; */
}

.firework::after {
  --x: -50%;
  --y: -50%;
  --initialY: -50%;
  /*   transform: translate(44vmin, -50%) rotate(170deg) scale(1.15) rotateY(-30deg); */
  transform: translate(-50%, -50%) rotate(170deg) scale(1.15) rotateY(-30deg);
  /*   animation: fireworkPseudo 2s infinite; */
}

.firework:nth-child(2) {
  --x: 30vmin;
}

.firework:nth-child(2),
.firework:nth-child(2)::before,
.firework:nth-child(2)::after {
  --color1: pink;
  --color2: violet;
  --color3: fuchsia;
  --color4: orchid;
  --color5: plum;
  --color6: lavender;
  --finalSize: 40vmin;
  left: 30%;
  top: 60%;
  animation-delay: -0.25s;
}

.firework:nth-child(3) {
  --x: -30vmin;
  --y: -50vmin;
}

.firework:nth-child(3),
.firework:nth-child(3)::before,
.firework:nth-child(3)::after {
  --color1: cyan;
  --color2: lightcyan;
  --color3: lightblue;
  --color4: PaleTurquoise;
  --color5: SkyBlue;
  --color6: lavender;
  --finalSize: 35vmin;
  left: 70%;
  top: 60%;
  animation-delay: -0.4s;
}
</style>
