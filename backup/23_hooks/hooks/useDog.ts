import { reactive, onMounted } from 'vue'
import axios from 'axios'



export default function () {
    let dogList = reactive([
        'https://images.dog.ceo/breeds/pointer-german/n02100236_5698.jpg'
    ])

    async function getDog() {
        try {
            let res = await axios.get('https://dog.ceo/api/breeds/image/random')
            dogList.push(res.data.message)
        } catch (error) {
            alert(error)
        }
    }
    onMounted(() => {
        getDog()
    })
    // 向外部
    return { dogList, getDog }
}