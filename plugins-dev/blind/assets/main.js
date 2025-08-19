// 这是一个简单的计数器函数
function counter() {
    let count = 0;
    
    return {
        increment: () => {
            count++;
            console.log(`当前计数: ${count}`);
        },
        decrement: () => {
            count--;
            console.log(`当前计数: ${count}`);
        },
        getCount: () => count
    };
}

// 创建一个新的计数器实例
const myCounter = counter();

// 测试计数器功能
myCounter.increment(); // 输出: 当前计数: 1
myCounter.increment(); // 输出: 当前计数: 2
myCounter.decrement(); // 输出: 当前计数: 1
console.log(`最终计数: ${myCounter.getCount()}`); // 输出: 最终计数: 1
hellp

被好几个好结果xxx收拾收拾sss