require 'set'
require_relative '../util'

class Day1
  def initialize
    @input = Util.read_input(1).split(', ')
  end

  def main
    puts(problem_one)
    puts(problem_two)
  end

  def move_directions
    dir = 1

    @input.map do |move|
      dir *= (move[0] == 'R' ? 1 : -1) * 1i
      [move[1..].to_i, dir]
    end
  end

  def distance(pos)
    pos.real.abs + pos.imag.abs
  end

  def problem_one
    distance(move_directions.map { |move, dir| move * dir }.sum)
  end

  def problem_two
    pos = 0
    visited = Set[pos]

    end_pos = catch(:done) do
      move_directions.each do |move, dir|
        (1..move).each do
          pos += dir
          throw(:done, pos) unless visited.add?(pos)
        end
      end
    end

    distance(end_pos)
  end
end

Day1.new.main
