<template>
  <div class="base_input flex">
    <div class="input_wrapper w100 flex">
      <input
        :class="{
          rounded: rounded,
          input_with_right_icon: rounded && rightIcon,
          slim: slim,
        }"
        class="mpx light_text"
        :type="type"
        v-model="value"
        :disabled="readonly"
      />
    </div>
    <div
      class="right_icon flex content_center align_center"
      @click="rightIconClicked()"
      :class="{
        right_icon_rounded: rounded,
        pointer: rightIconAction,
        disabled: rightIconAction && !value,
      }"
      v-if="rightIcon"
    >
      <span class="material-symbols-outlined dark_text">
        {{ rightIconText }}
      </span>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "BaseInput",

  props: {
    rounded: {
      type: Boolean,
      default: true,
    },

    slim: {
      type: Boolean,
      default: false,
    },

    readonly: {
      type: Boolean,
      default: false,
    },

    rightIcon: {
      type: Boolean,
      default: false,
    },

    rightIconAction: {
      type: Boolean,
      default: false,
    },

    rightIconText: {
      type: String,
      default: "",
    },

    type: {
      type: String,
      default: "text",
      validator: function (value: string) {
        return ["text", "number", "email"].includes(value);
      },
    },
  },

  data() {
    return {
      value: "" as string,
    };
  },

  watch: {
    value() {
      this.$emit('valueUpdated', this.value)
    },
  },

  methods: {
    setValue(value: string) {
      this.value = value;
    },

    focus() {

    },

    rightIconClicked() {
      this.$emit("rightIconClicked");
    },
  },
});
</script>

<style scoped lang="scss">
.base_input {
  width: 100%;
  height: 100%;

  input {
    width: 100%;
    height: 100%;
    background-color: transparent;
    border: none;
    outline: none;
  }

  .right_icon {
    min-width: 40px;
    font-size: 4em;
    background-color: #6deeff88;

    .material-symbols-outlined {
      font-size: 0.5em;
    }
  }

  .input_with_right_icon {
    border-radius: 12px 0 0 12px;
  }

  .right_icon_rounded {
    border-radius: 0 12px 12px 0;
  }
}
</style>
