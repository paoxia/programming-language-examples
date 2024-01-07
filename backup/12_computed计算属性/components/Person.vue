<template>
    <div class="person">
        firstName : <input type="text" v-model="firstName"> <br>
        lastName : <input type="text" v-model="lastName"> <br>
        fullName : <span>{{ fullName }}</span>
        <hr>
        <button @click="changeFullName">modifyFullName</button>
    </div>
</template>



<script lang="ts" setup name="Person">

import { ref, computed } from 'vue';

let firstName = ref('san')
let lastName = ref('zhang')

// let fullName = computed(() => {
//     return firstName.value.slice(0, 1).toUpperCase() + firstName.value.slice(1) + "-" + lastName.value;
// }) // computer带缓存 这么定义只读 不能修改

let fullName = computed({

    get() {
        return firstName.value.slice(0, 1).toUpperCase() + firstName.value.slice(1) + "-" + lastName.value;
    },

    set(val) {
        console.log('set', val)
        const [_firstName, _lastName] = val.split('-')
        firstName.value = _firstName
        lastName.value = _lastName
    }
})


function changeFullName() {
    fullName.value = 'si-li'
}



</script>


<style scoped >
.person {
    box-shadow: 0 0 10px;
    border-radius: 10px;
    padding: 20px;
}

button {
    margin: 0 5px;
}
</style>
