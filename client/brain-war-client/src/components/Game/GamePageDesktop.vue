<template>
  <div class="game_left_sidebar dark_background_trans">
    <LeftSidebar
      :headers="playersListHeaders"
      :records="$store.getters.getPlayersStatus ?? []"
    />
  </div>
  <div class="game_center_bar dark_background">
    <GameActionTable
      :headers="actionsListHeaders"
      :records="$store.getters.getPlayersActions ?? []"
    />
  </div>
  <div class="game_right_sidebar dark_background_trans">
    <GameChat />
  </div>
  <GameDisplay />
  <div
    class="game_button_wrapper"
    :class="{
      game_button_wrapper_animated: isGameButtonShaking,
    }"
  >
    <GameButton
      :text="buttonText"
      :animation="showAnimation"
      :isHidden="isButtonHidden"
      :disabled="isButtonDisabled"
      @submit="submit()"
    />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "@vue/runtime-core";
import LeftSidebar from "./LeftSidebar.vue";
import GameButton from "@/components/Game/GameButton.vue";
import GameActionTable from "@/components/Game/GameActionTable.vue";
import GameChat from "@/components/Game/GameChat.vue";
import GameDisplay from "@/components/Game/GameDisplay.vue";
import Store from "@/store"; // path to store file

declare module "@vue/runtime-core" {
  interface ComponentCustomProperties {
    $store: typeof Store;
  }
}

export default defineComponent({
  name: "GamePageDesktop",

  components: {
    LeftSidebar,
    GameButton,
    GameActionTable,
    GameChat,
    GameDisplay,
  },

  data() {
    return {
      actionsListHeaders: [
        { text: "Player", value: "name" },
        { text: "Action", value: "action" },
        { text: "Time", value: "created_at" },
      ],
      playersListHeaders: [
        { text: "Player", value: "name" },
        { text: "Status", value: "status" },
      ],
    };
  },

  computed: {
    isGameButtonShaking(): boolean {
      return this.$store.getters.isGameButtonShaking;
    },

    buttonText(): string {
      if (this.$store.getters.isBailed) {
        return "BAILED";
      }

      if (this.$store.getters.isGameLive && this.$store.getters.isPlaying) {
        return "BAIL";
      }

      if (!this.$store.getters.isGameLive && this.$store.getters.isPlaying) {
        return "READY";
      }

      return "JOIN";
    },

    isButtonHidden(): boolean {
      return this.$store.getters.isGameLive && !this.$store.getters.isPlaying;
    },

    isButtonDisabled(): boolean {
      return (
        this.$store.getters.isGameFinished ||
        (!this.$store.getters.isGameLive && this.$store.getters.isPlaying) ||
        this.$store.getters.isBailed ||
        (this.$store.getters.isPlaying && !this.$store.getters.isGameLive)
      );
    },

    showAnimation(): boolean {
      return (
        this.$store.getters.isGameAboutToStart && !this.$store.getters.isPlaying
      );
    },
  },

  methods: {
    submit() {
      this.$store.getters.isGameLive
        ? this.$store.dispatch("bailGame")
        : this.$store.dispatch("joinGame");
    },
  },
});
</script>

<style scoped lang="scss">
.game_button_wrapper {
  position: fixed;
  height: 20vw;
  width: 20vw;
  border-radius: 50%;
  bottom: 8vh;
  left: 40vw;

  @media only screen and (max-width: 1700px) {
    width: 24vw;
    left: 38vw;
  }
}

.game_button_wrapper_animated {
  animation: shake 0.5s;

  @keyframes shake {
    0%,
    100% {
      transform: translateX(0);
    }
    10%,
    50%,
    90% {
      transform: translateX(-5px);
    }
    30%,
    70% {
      transform: translateX(5px);
    }
  }
}

.game_left_sidebar,
.game_right_sidebar {
  position: fixed;
  bottom: 0;
  height: 75vh;
  width: 20vw;
  border-top: 2px solid #e0ded2;
  z-index: 2;
}

.game_left_sidebar {
  left: 0;
  border-radius: 0 12px 0 0;
  border-right: 2px solid #e0ded2;
  box-shadow: 4px 8px 8px 3px #00011088;
}

.game_right_sidebar {
  right: 0;
  border-radius: 12px 0 0 0;
  border-left: 2px solid #e0ded2;
  box-shadow: -4px 8px 8px 3px #00011088;

  .sidebar_user_name_wrapper {
    height: 4%;
    position: relative;
    width: 90%;
    left: 5%;
    margin-top: 5%;
  }
}

.game_center_bar {
  position: fixed;
  bottom: 0;
  width: 60vw;
  left: 20vw;
  height: 25vh;
  border-top: 2px solid #e0ded2;
  box-shadow: 0px -8px 8px 3px #00011088;
  z-index: 1;
}
</style>
