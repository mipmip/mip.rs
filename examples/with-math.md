---
title: Math Examples
---

# TeX Math in Markdown

## Inline Math

The quadratic formula gives $x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$ for any $ax^2 + bx + c = 0$.

Euler's identity $e^{i\pi} + 1 = 0$ connects five fundamental constants. The golden ratio is $\varphi = \frac{1 + \sqrt{5}}{2} \approx 1.618$.

## Display Math

The Gaussian integral:

$$\int_{-\infty}^{\infty} e^{-x^2} \, dx = \sqrt{\pi}$$

Maxwell's equations in differential form:

$$\nabla \cdot \mathbf{E} = \frac{\rho}{\varepsilon_0}$$

$$\nabla \cdot \mathbf{B} = 0$$

$$\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t}$$

$$\nabla \times \mathbf{B} = \mu_0 \mathbf{J} + \mu_0 \varepsilon_0 \frac{\partial \mathbf{E}}{\partial t}$$

## Summations and Series

The Basel problem:

$$\sum_{n=1}^{\infty} \frac{1}{n^2} = \frac{\pi^2}{6}$$

Taylor expansion of $e^x$:

$$e^x = \sum_{n=0}^{\infty} \frac{x^n}{n!} = 1 + x + \frac{x^2}{2!} + \frac{x^3}{3!} + \cdots$$

## Matrices

A rotation matrix in 2D:

$$R(\theta) = \begin{pmatrix} \cos\theta & -\sin\theta \\ \sin\theta & \cos\theta \end{pmatrix}$$

## Cases and Piecewise

The absolute value function:

$$|x| = \begin{cases} x & \text{if } x \geq 0 \\ -x & \text{if } x < 0 \end{cases}$$

## Greek Letters and Operators

The probability density of the normal distribution with mean $\mu$ and standard deviation $\sigma$:

$$f(x) = \frac{1}{\sigma\sqrt{2\pi}} \exp\left(-\frac{(x - \mu)^2}{2\sigma^2}\right)$$

## Math in Context

Consider a function $f: \mathbb{R} \to \mathbb{R}$. If $f$ is differentiable at $a$, then:

$$f'(a) = \lim_{h \to 0} \frac{f(a+h) - f(a)}{h}$$

This should **not** render as math: the price is $5 and the total is $10. Dollar signs in `$inline code$` or in fenced code blocks should be left alone:

```
$not math$
$$also not math$$
```
