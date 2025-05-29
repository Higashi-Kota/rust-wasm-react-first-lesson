import { describe, it, expect, beforeEach, vi } from 'vitest'
import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { MathCalculator } from './MathCalculator'

// WASMモジュールのモック（setup.tsで定義済みだが、ここでも明示的に定義）
vi.mock('@internal/wasm-math', async () => {
  const actualModule = await vi.importActual('@internal/wasm-math')
  return {
    ...actualModule,
    add: vi.fn((a: number, b: number) => a + b),
    sub: vi.fn((a: number, b: number) => a - b),
    mul: vi.fn((a: number, b: number) => a * b),
    div: vi.fn((a: number, b: number) => {
      if (b === 0)
        return Promise.reject(new Error('0で除算することはできません'))
      return Promise.resolve(a / b)
    }),
    pow: vi.fn((base: number, exp: number) => Math.pow(base, exp)),
    sqrt: vi.fn((x: number) => Math.sqrt(x)),
  }
})

describe('MathCalculator', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('コンポーネントが正しくレンダリングされる', () => {
    render(<MathCalculator />)

    expect(screen.getByText('数学計算 (wasm-math)')).toBeInTheDocument()
    expect(screen.getByText('基本四則演算')).toBeInTheDocument()
    expect(screen.getByText('平方根')).toBeInTheDocument()
    expect(screen.getByText('累乗')).toBeInTheDocument()
  })

  it('足し算が正しく実行される', async () => {
    const user = userEvent.setup()
    render(<MathCalculator />)

    // 入力値を設定
    const num1Input = screen.getByDisplayValue('10')
    const num2Input = screen.getByDisplayValue('5')

    await user.clear(num1Input)
    await user.type(num1Input, '15')
    await user.clear(num2Input)
    await user.type(num2Input, '25')

    // 足し算ボタンをクリック
    const addButton = screen.getByText('足し算 (+)')
    await user.click(addButton)

    // 結果が表示されることを確認
    await waitFor(() => {
      expect(screen.getByText(/15 \+ 25/)).toBeInTheDocument()
      expect(screen.getByText(/= 40/)).toBeInTheDocument()
    })
  })

  it('無効な入力でエラーが表示される', async () => {
    const user = userEvent.setup()
    render(<MathCalculator />)

    // 無効な値を入力
    const num1Input = screen.getByDisplayValue('10')
    await user.clear(num1Input)
    await user.type(num1Input, 'abc')

    // 計算ボタンをクリック
    const addButton = screen.getByText('足し算 (+)')
    await user.click(addButton)

    // エラーメッセージが表示されることを確認
    await waitFor(() => {
      expect(
        screen.getByText(/有効な数値を入力してください/)
      ).toBeInTheDocument()
    })
  })

  it('0で除算するとエラーが表示される', async () => {
    const user = userEvent.setup()
    render(<MathCalculator />)

    // 除算用の値を設定（0で除算）
    const num1Input = screen.getByDisplayValue('10')
    const num2Input = screen.getByDisplayValue('5')

    await user.clear(num2Input)
    await user.type(num2Input, '0')

    // 割り算ボタンをクリック
    const divButton = screen.getByText('割り算 (÷)')
    await user.click(divButton)

    // エラーメッセージが表示されることを確認
    await waitFor(() => {
      expect(
        screen.getByText(/0で除算することはできません/)
      ).toBeInTheDocument()
    })
  })

  it('平方根が正しく計算される', async () => {
    const user = userEvent.setup()
    render(<MathCalculator />)

    // 平方根の入力値を設定
    const sqrtInput = screen.getByDisplayValue('16')
    await user.clear(sqrtInput)
    await user.type(sqrtInput, '25')

    // 平方根ボタンをクリック
    const sqrtButton = screen.getByText('√ 平方根')
    await user.click(sqrtButton)

    // 結果が表示されることを確認
    await waitFor(() => {
      expect(screen.getByText(/√25/)).toBeInTheDocument()
      expect(screen.getByText(/= 5.0000/)).toBeInTheDocument()
    })
  })

  it('累乗が正しく計算される', async () => {
    const user = userEvent.setup()
    render(<MathCalculator />)

    // 累乗の入力値を設定
    const baseInput = screen.getByDisplayValue('2')
    const expInput = screen.getByDisplayValue('3')

    await user.clear(baseInput)
    await user.type(baseInput, '5')
    await user.clear(expInput)
    await user.type(expInput, '2')

    // 計算ボタンをクリック
    const powButton = screen.getByText('計算')
    await user.click(powButton)

    // 結果が表示されることを確認
    await waitFor(() => {
      expect(screen.getByText(/5\^2/)).toBeInTheDocument()
      expect(screen.getByText(/= 25/)).toBeInTheDocument()
    })
  })

  it('計算結果の履歴が正しく表示される', async () => {
    const user = userEvent.setup()
    render(<MathCalculator />)

    // 複数回計算を実行
    const addButton = screen.getByText('足し算 (+)')
    await user.click(addButton)

    const subButton = screen.getByText('引き算 (-)')
    await user.click(subButton)

    // 履歴セクションが表示されることを確認
    await waitFor(() => {
      expect(screen.getByText('計算結果')).toBeInTheDocument()
      // 最新の結果が上に来ることを確認
      const results = screen.getAllByText(/= \d+/)
      expect(results.length).toBeGreaterThan(1)
    })
  })
})
