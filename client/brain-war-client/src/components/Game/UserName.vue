<template>
  <div class="user_name white_text flex align_center content_space_between">
    <div class="flex align_center">
      <Icon v-if="!isEditable" :pointer="false" small icon="account_circle" color="#e0ded2" />
      <Icon v-else small icon="cancel" color="#e0ded2" @clicked="cancel()" />
      <form ref="chatForm" class="h100" @submit.prevent="submitName()">
        <BaseInput :readonly="!isEditable" ref="input" @valueUpdated="valueUpdated" />
      </form>
    </div>
    <div class="flex align_center">
      <Icon v-if="!isEditable" small icon="edit" dark @clicked="toggleEdit()" />
      <Icon v-else small icon="send" dark @clicked="submitName()" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import Icon from "../Shared/Global/Icon.vue";
import BaseInput from "../Shared/Inputs/BaseInput.vue";

export default defineComponent({
  name: "UserName",

  components: { 
    Icon,
    BaseInput,
   },

  data() {
    return {
      isEditable: false,
      userName: ''
    };
  },

  mounted() {
    this.setCurrentName()
  },

  watch: {
    playerName() {
      this.setCurrentName();
    }
  },

  computed: {
    playerName() {
      return this.$store.getters.getPlayerName;
    }
  },

  methods: {
    setCurrentName() {
      (this.$refs.input as any).setValue(this.playerName);
    },

    toggleEdit() {
      this.$store.dispatch("playAudio", 'click');
      this.isEditable = !this.isEditable; 
      if(this.isEditable) {
        (this.$refs.input as HTMLDivElement).focus();
      }
    },
  
    cancel() {
      this.toggleEdit();
      this.setCurrentName();
    },

    submitName() {
      this.toggleEdit();

      if(!this.userName.trim() || this.userName === this.playerName) {
        return;
      }

      this.$store.dispatch('updatePlayerName', this.userName.trim())
    },

    valueUpdated(value: string) {
      this.userName = value;
    } 
  },
});
</script>

<style scoped lang="scss">
.user_name {
  width: calc(100% - 24px);
  height: 100%;
  border-radius: 22px;
  padding: 0px 12px;
}
</style>
