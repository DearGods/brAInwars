<template>
  <dialog open class="game_deposit_dialog mp">
    <div>
      <Icon small icon="close" dark @clicked="closeModal()" />
    </div>
    <p v-html="description" class="white_text"></p>
    <div
      class="user_name light_background_trans white_text flex align_center content_space_between"
    >
      <div class="flex align_center w100">
        <form
          ref="chatForm"
          class="h100 w100"
          @submit.prevent="submitDepositPrice()"
        >
          <BaseInput type="number" ref="input" @valueUpdated="valueUpdated" />
        </form>
      </div>
      <div class="flex align_center">
        <Icon small icon="send" dark @clicked="submitDepositPrice()" />
      </div>
    </div>
  </dialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import Icon from "../Shared/Global/Icon.vue";
import BaseInput from "../Shared/Inputs/BaseInput.vue";
import { DepositModal, DepositModalType } from "@/helpers/feTypes";

export default defineComponent({
  name: "GameDepositModal",

  components: {
    Icon,
    BaseInput,
  },

  data() {
    return {
      isEditable: false,
      depositPrice: 0 as number,
    };
  },

  mounted() {
    if (
      this.$store.getters.getDepositModalMode === DepositModal.MISSING_MONEY
    ) {
      (this.$refs.input as any).setValue(
        (this.$store.getters.getGamePrice /
          this.$store.getters.getGameCoinCalc) *
          10,
      );
    }
  },

  computed: {
    description() {
      const text = {} as Record<DepositModalType, string>;
      text[DepositModal.MISSING_MONEY] = `
      You don't have enough money in your wallet to join the game.
      <br>
      Can deposit the filled number or change it as you like.`;
      text[DepositModal.ADDING_MONEY] = `
      You can deposit any amount of money you would like ahead.
      <br>
      It is always good to be prepared.`;
      text[DepositModal.WITHDRAW] = `
      You are always welcome to withdraw money back.
      <br>
      Enter the number you would like to draw.`;
      return text[this.$store.getters.getDepositModalMode as DepositModalType];
    },
  },

  methods: {
    submitDepositPrice() {
      if (this.$store.getters.getDepositModalMode === DepositModal.WITHDRAW) {
        this.$store.dispatch("setWithdrawPrice", this.depositPrice);
      } else {
        this.$store.dispatch("setDepositPrice", this.depositPrice);
      }
    },

    valueUpdated(value: number) {
      this.depositPrice = value;
    },

    closeModal() {
      this.$store.dispatch("playAudio", "click");
      this.$store.dispatch("setDepositModalMode", DepositModal.CLOSED);
    },
  },
});
</script>

<style scoped lang="scss">
.game_deposit_dialog {
  position: absolute;
  top: 22%;
  left: 0;
  right: 0;
  margin: auto;
  height: 230px;
  width: 460px;
  background-color: var(--darkBlue);
  box-shadow: 0 0 8px 3px #0008;
  max-width: calc(100% - 40px);
  z-index: 2;
}

.user_name {
  margin-top: 80px;
  width: calc(100% - 20px);
  padding: 10px;
  border-radius: 12px;
}
</style>
