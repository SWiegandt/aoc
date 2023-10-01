require_relative '../util'

class Day6
  def initialize
    @input = Util.read_input(6).split("\n").map { |row| row.split('') }.transpose
  end

  def main
    puts(problem_one)
    puts(problem_two)
  end

  def most_common(arr)
    arr.group_by(&:itself).transform_values(&:length).min_by { |_, value| yield value }[0]
  end

  def problem_one
    @input.map { |col| most_common(col, &:-@) }.join('')
  end

  def problem_two
    @input.map { |col| most_common(col, &:itself) }.join('')
  end
end

Day6.new.main
