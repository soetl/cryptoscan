<script setup>
import { ref } from "vue";

const dialog = defineModel();
const props = defineProps({
  coins: {
    type: Array,
    required: true,
  },
});
const emit = defineEmits(["submit"]);

const selectedCoins = ref([]);

function map(coins) {
  console.log("Mapping coins: ", coins);
  return coins.map((coin) => coin.name);
}

function submit() {
  emit("submit", props.coins.filter((coin) => selectedCoins.value.includes(coin.name)));
  dialog.value = false;
  selectedCoins.value = [];
}
</script>

<template>
  <v-dialog v-model="dialog">
    <v-card>
      <v-card-title title="Choose Coins" />
      <v-card-text>
        <v-select
          v-model="selectedCoins"
          :items="coins.map((coin) => coin.name)"
          label="Coins"
          multiple
          chips
        />
      </v-card-text>
      <v-card-actions>
        <v-btn @click="dialog = false" text="Cancel" />
        <v-spacer />
        <v-btn @click="submit" text="Submit" />
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>
