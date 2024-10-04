<template>
  <div class="game_page_mobile_wrapper">
    <template v-if="$store.getters.getChatMode">
      <div class="rounded dark_background_trans game_mobile_chat_wrapper">
        <GameChat />
      </div>
    </template>
    <template v-else>
    <GameDisplay/>
    <!-- <div class="game_mobile_room_info_row dark_background_trans">
      <div v-for="(item, index) in stats" :key="index">
        <Icon small dark :icon="item.icon" color="#e0ded2" class="icon wallet_icon" />
        <small class="white_text">{{ item.value }}</small>
      </div>
    </div> -->
      <div class="game_page_mobile_data_tables rounded dark_background_trans">
        <div class="game_mobile_data_tabs_wrapper">
          <GameTabs :tabs="tabs" @tabPicked="setRoute" />
        </div>
        <div
          class="white_color game_mobile_data_content_wrapper dark_background_trans white_text"
        >
          <GameActionTable
            v-if="$route.query.tab == tabs[0].toLowerCase()"
            :headers="playersListHeaders"
            :records="playersStatus"
          />
          <GameActionTable
            v-else
            :headers="actionsListHeaders"
            :records="$store.getters.getPlayersActions ?? []"
          />
        </div>
      </div>
    </template>
    <div class="game_button_wrapper">
      <GameButton
        :text="buttonText"
        :animation="showAnimation"
        :isHidden="isButtonHidden"
        :disabled="isButtonDisabled"
        @submit="submit()"
      />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "@vue/runtime-core";
import GameButton from "@/components/Game/GameButton.vue";
import GameActionTable from "@/components/Game/GameActionTable.vue";
import GameTabs from "@/components/Game/GameTabs.vue";
import GameDisplay from "@/components/Game/GameDisplay.vue";
import GameChat from "./GameChat.vue";
import Store from "@/store"; // path to store file
import { PLAYER_STATUS_TEXT } from "@/helpers/enums/PlayerStatusEnum";
import { GamePlayersStatus, PLAYER_STATUS } from "@/helpers/apiTypes";
import Icon from '../../components/Shared/Global/Icon.vue';

declare module "@vue/runtime-core" {
  interface ComponentCustomProperties {
    $store: typeof Store;
  }
}

export default defineComponent({
  name: "GamePageMobile",

  components: {
    GameButton,
    GameActionTable,
    GameTabs,
    GameDisplay,
    GameChat,
    Icon,
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
    gamePrice(): string {
      return "$" + this.$store.getters.getGamePrice;
    },

    imageSrc(): string {
      return "https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/395631ff-ba94-42ca-4654-8a70e524e800/public";
    },

    stats(): Record<string, string>[] {
      return [
        // {
        //   text: "Balance",
        //   value: this.$store.getters.getUserBalance ? ("$" + this.$store.getters.getUserBalance) : '',
        // },
        {
          icon: "payments",
          value: "$" + this.$store.getters.getGamePrice,
        },
        {
          icon: "savings",
          value:
            "$" +
            this.$store.getters.getActivePlayers?.length *
              this.$store.getters.getGamePrice,
        },
        {
          icon: "groups",
          value: this.$store.getters.getActivePlayers?.length,
        },
      ];
    },

    playersStatus() {
      return this.$store.getters.getPlayersStatus.map((record: GamePlayersStatus) => {
        let status = PLAYER_STATUS_TEXT.READY;
        if(this.$store.getters.isGameLive && record.player_status === PLAYER_STATUS.JOINED_GAME) {
          status = PLAYER_STATUS_TEXT.PLAYING;
        }
        
        if(record.player_status === PLAYER_STATUS.LEFT_GAME) {
          status = PLAYER_STATUS_TEXT.BAILED;
        }
        
        return {
          name: record.wallet_address === this.$store.getters.getWalletAddress ? 'SELF' : (record?.name ?? 'PLAYER'),
          status: status
        }
      }) ?? []
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

    tabs(): Array<string> {
      return ["PLAYERS", "HISTORY"];
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

    setRoute(tab: string) {
      this.$router.push({
        path: this.$route.path,
        query: { tab: tab.toLowerCase() },
      });
    },
  },
});
</script>

<style scoped lang="scss">
.game_page_mobile_wrapper {
  position: absolute;
  height: calc(100% - 80px);
  width: 100vw;
  overflow-y: hidden;

  // .game_mobile_room_info_row {
  //   position: absolute;
  //   top: 20%;
  //   width: calc(90% - 8px);
  //   left: 5%;
  //   height: 6%;
  //   border-radius: 12px;
  //   display: flex;
  //   justify-content: space-around;
  //   padding: 4px;

  //   div {
  //     text-align: center;
  //     span {
  //       display: block;
  //     }
  //   }
  // }

  .game_page_mobile_data_tables {
    position: absolute;
    top: 22%;
    left: 5%;
    height: calc(53% - 31px);
    width: 90%;
  }

  .game_mobile_chat_wrapper {
    position: absolute;
    top: 1%;
    left: 5%;
    height: 70%;
    width: 90%;
  }

  .game_mobile_data_content_wrapper {
    height: 80%;
    overflow-y: auto;
    top: 10px;
    position: relative;
    border-radius: 12px;
  }

  .game_button_wrapper {
    position: absolute;
    bottom: -205px;
    left: 0;
    right: 0;
    margin: auto;
    width: 94%;
  }

  .game_mobile_data_tabs_wrapper {
    height: 20%;
    position: relative;
  }
}
</style>
