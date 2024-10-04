<template>
  <div class="">
    <table border="1">
      <tbody>
        <tr>
          <td colspan="999">
            <div v-if="gameId">GameId: {{ gameId }}</div>
            <input type="text" v-model="gameId" />
          </td>
        </tr>
        <tr>
          <td>
            <button class="btn btn-primary" @click="credFlow">
              Connect & Register & login
            </button>
          </td>
          <td>
            <button class="btn btn-primary" @click="getGames">getGames</button>
          </td>
          <td>
            <button class="btn btn-primary" @click="getGame">getGame</button>
          </td>
          <td>
            <button class="btn btn-primary" @click="deposit">deposit</button>
          </td>
          <td>
            <button class="btn btn-primary" @click="withdraw">withdraw</button>
          </td>
          <td>
            <button class="btn btn-primary" @click="getChainBalance">
              getBalance
            </button>
          </td>
          <td>
            <button class="btn btn-primary" @click="joinGame">joinGame</button>
          </td>
          <td>
            <button class="btn btn-primary" @click="leaveGame">
              leaveGame
            </button>
          </td>
          <td>
            <button class="btn btn-primary" @click="connect_ws">
              Connect WebSockets
            </button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

export default defineComponent({
  name: "TestPage",

  computed: {},

  data() {
    return {
      gameId: "",
    };
  },

  methods: {
    async connect_ws() {
      this.$store.dispatch("connectWs");
    },

    async leaveGame() {
      this.$store.dispatch("leaveGameReal", this.gameId);
    },

    async joinGame() {
      this.$store.dispatch("joinGameReal", this.gameId);
    },

    async credFlow() {
      await this.$store.dispatch("connectPhantom", (window as any)?.solana);
      // await this.$store.dispatch("connectPhantom", (window as any)?.backpack);
      await this.$store.dispatch("register");
      await this.$store.dispatch("login");
      await this.$store.dispatch("connectWs");
    },

    async getGames() {
      this.$store.dispatch("getGames");
    },

    async getGame() {
      this.$store.dispatch("getGame", this.gameId);
    },

    async deposit() {
      this.$store.dispatch("deposit", LAMPORTS_PER_SOL / 100);
    },

    async withdraw() {
      this.$store.dispatch("withdraw", LAMPORTS_PER_SOL / 100);
    },

    async getChainBalance() {
      this.$store.dispatch("getChainBalance");
    },
  },
});
</script>

<style scoped lang="scss">
.home_page {
  height: calc(100vh - 80px);
  width: 100vw;
  position: relative;

  .background_image {
    height: 100%;
    width: 100%;
    object-fit: cover;
    position: absolute;
  }

  .home_content {
    height: 50vh;
    width: 35vw;
    position: relative;
    top: -10px;
    justify-content: space-between;
  }
}
</style>
