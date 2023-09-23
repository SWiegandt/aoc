require_relative '../util'

class Day3
  def initialize
    @input = Util.read_input(3).lines.map { |row| row.strip.split.map(&:to_i) }
  end

  def main
    puts(problem_one)
    puts(problem_two)
  end

  def valid_triangles(input)
    input.map(&:sort).filter { |(a, b, c)| a + b > c }.count
  end

  def problem_one
    valid_triangles(@input)
  end

  def problem_two
    valid_triangles(@input.transpose.flatten.each_slice(3))
  end
end

Day3.new.main
