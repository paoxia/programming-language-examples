import { ref, onMounted, computed } from 'vue'
import axios from 'axios'


export default function () {
    let sum = ref(0)
    let bigSum = computed(() => {
        return sum.value * 10
    })
    function add() {
        sum.value += 1
    }
    onMounted(() => {
        add()
    })
    // 向外部
    return { sum, add, bigSum }
}