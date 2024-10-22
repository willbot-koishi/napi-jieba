import test from 'ava'

import { Jieba } from '..'

const jieba = new Jieba()

test('Jieba#cut', t => {
	t.deepEqual(jieba.cut('你好世界'), ['你好', '世界'])
})

test('Jieba#cutAll', t => {
	t.deepEqual(jieba.cutAll('南京市长江大桥'), [
		'南', '南京',
		'南京市', '京',
		'京市', '市',
		'市长', '长',
		'长江', '长江大桥',
		'江', '大',
		'大桥', '桥'
	])
})
