<template>
  <div class="game_left_sidebar">
    <div class="game_left_content rounded light_background_trans">
      <div class="game_stats mp flex column content_space_between rounded">
        <div
          v-for="(item, index) in stats"
          :key="index"
          class="game_stat_record flex content_space_around white_text"
        >
          <div class="w100 text_left">
            {{ item.text }}
          </div>
          <span class="text_right">
            {{ item.value }}
          </span>
        </div>
      </div>

      <div class="game_players_list rounded">
        <TableComponent :headers="headers" :records="playersStatus" />
      </div>

      <div class="user_name_wrapper rounded">
        <UserName
          :class="{
            disabled: !$store.getters.isLoggedIn,
          }"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import TableComponent from "../../components/Shared/Table/Table.vue";
import IGameStat from "@/helpers/interfaces/IGameStat";
import { PLAYER_STATUS_TEXT } from "@/helpers/enums/PlayerStatusEnum";
import IHeader from "@/helpers/interfaces/ITableHeader";
import { GamePlayersStatus, PLAYER_STATUS } from "@/helpers/apiTypes";
import UserName from "@/components/Game/UserName.vue";

export default defineComponent({
  name: "LeftSidebar",

  components: {
    TableComponent,
    UserName,
  },

  props: {
    headers: {
      type: Array as PropType<IHeader[]>,
      required: true,
    },
    records: {
      type: Array as PropType<GamePlayersStatus[]>,
      required: true,
    },
  },

  data() {
    return {};
  },

  computed: {
    stats(): IGameStat[] {
      return [
        {
          text: "Balance",
          value: this.$store.getters.getUserBalance ? ("$" + this.$store.getters.getUserBalance) : '',
        },
        {
          text: "Entry Fee",
          value: "$" + this.$store.getters.getGamePrice,
        },
        {
          text: "Total Pot",
          value:
            "$" +
            this.$store.getters.getActivePlayers?.length *
              this.$store.getters.getGamePrice,
        },
        // {
        //   text: "Total Players",
        //   value: this.$store.getters.getTotalPlayers,
        // },
        {
          text: "Active Players",
          value: this.$store.getters.getActivePlayers?.length,
        },
      ];
    },
    
    playersStatus() {
      return this.records.map(record => {
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
    }
  },
});
</script>

<style scoped lang="scss">
.game_left_sidebar {
  width: 100%;
  height: 100%;

  .game_left_content {
    position: relative;
    height: 100%;
    width: 90%;
    margin-top: 7%;
    margin-left: 5%;

    .game_stats {
      margin-left: 3%;
      width: calc(94% - 24px);
      height: calc(20% - 24px);
      background-color: #000110aa;
      position: relative;
      top: 2%;
      // min-height: 170px;

      .game_stat_record {
        font-size: 0.8em;

        .text_right {
          width: 30%;
        }
      }
    }

    .game_players_list {
      margin-left: 3%;
      width: 94%;
      height: 60%;
      background-color: #000110e8;
      position: relative;
      top: 5%;
    }

    .user_name_wrapper {
      margin-left: 3%;
      width: 94%;
      height: 7%;
      background-color: #000110e8;
      position: relative;
      top: 7%;
    }
  }
}
</style>
