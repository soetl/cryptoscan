<script setup>
import { ref, computed } from "vue";

import { mdiDelete } from "@mdi/js";

const props = defineProps({
  coin: {
    type: Object,
    required: true,
  },
  editMode: {
    type: Boolean,
    required: true,
  },
});

const emit = defineEmits(["removeCoin"]);

const minimizedPriceSize = computed(() => {
  return props.coin.price > 0.01 ? props.coin.price.toFixed(2) : props.coin.price;
});

const percentColor = computed(() => {
  return props.coin.percent_change_24h > 0 ? "green" : "red";
});
</script>

<template>
  <v-card class="mx-2 mb-2" :title="coin.symbol + ' - ' + coin.name">
    <template v-slot:prepend>
      <v-img
        width="42"
        :aspect-ratio="1 / 1"
        :src="
          'https://s2.coinmarketcap.com/static/img/coins/64x64/' +
          coin.id +
          '.png'
        "
        cover
        :alt="coin.symbol"
      />
    </template>
    <template v-slot:append>
      <v-btn
        icon
        density="comfortable"
        v-if="props.editMode"
        @click="emit('removeCoin', coin.id, coin.symbol)"
        variant="text"
      >
        <v-icon color="error" :icon="mdiDelete" />
      </v-btn>
      <!-- <p v-else>{{ coin.last_updated }}</p> -->
    </template>
    <v-card-text class="d-flex">
      {{ minimizedPriceSize }}$
      <v-spacer />
      <span :style="{ color: percentColor }">{{
        coin.percent_change_24h.toFixed(2)
      }}%</span>
    </v-card-text>
  </v-card>
</template>
